use std::cmp::Ordering;

use credit::{AgeAmount, AgeAmounts, InitialState};
use linera_sdk::{
    base::{Amount, ApplicationId, Owner, Timestamp},
    contract::system_api::current_system_time,
    views::{MapView, RegisterView, SetView, ViewStorageContext},
};
use linera_views::views::{GraphQLView, RootView};
use thiserror::Error;

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct Credit {
    pub initial_supply: RegisterView<Amount>,
    pub balance: RegisterView<Amount>,
    pub amount_alive_ms: RegisterView<u64>,
    pub balances: MapView<Owner, AgeAmounts>,
    pub spendables: MapView<Owner, Amount>,
    pub callers: SetView<ApplicationId>,
}

#[allow(dead_code)]
impl Credit {
    pub(crate) async fn initialize(&mut self, state: InitialState) {
        self.initial_supply.set(state.initial_supply);
        self.balance.set(state.initial_supply);
        self.amount_alive_ms.set(state.amount_alive_ms);
    }

    pub(crate) async fn initial_supply(&self) -> Amount {
        *self.initial_supply.get()
    }

    pub(crate) async fn balance(&self, owner: Option<Owner>) -> Amount {
        match owner {
            Some(owner) => self.balances.get(&owner).await.unwrap().unwrap().sum(),
            None => *self.balance.get(),
        }
    }

    pub(crate) async fn reward(&mut self, owner: Owner, amount: Amount) -> Result<(), StateError> {
        match self.spendables.get(&owner).await {
            Ok(Some(spendable)) => {
                self.spendables
                    .insert(&owner, spendable.saturating_add(amount))
                    .unwrap();
            }
            _ => {
                self.spendables.insert(&owner, amount).unwrap();
            }
        }

        log::info!(
            "Supply balance {} reward amount {}",
            self.balance.get(),
            amount
        );
        match self.balance.get().cmp(&amount) {
            Ordering::Less => return Err(StateError::InsufficientSupplyBalance),
            _ => {}
        }

        self.balance.set(self.balance.get().saturating_sub(amount));

        match self.balances.get(&owner).await {
            Ok(Some(mut amounts)) => {
                amounts.amounts.push(AgeAmount {
                    amount,
                    expired: Timestamp::from(
                        current_system_time()
                            .micros()
                            .saturating_add(*self.amount_alive_ms.get()),
                    ),
                });
                match self.balances.insert(&owner, amounts) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(StateError::ViewError(err)),
                }
            }
            _ => match self.balances.insert(
                &owner,
                AgeAmounts {
                    amounts: vec![AgeAmount {
                        amount,
                        expired: Timestamp::from(
                            current_system_time()
                                .micros()
                                .saturating_add(*self.amount_alive_ms.get()),
                        ),
                    }],
                },
            ) {
                Ok(_) => Ok(()),
                Err(err) => Err(StateError::ViewError(err)),
            },
        }
    }

    pub(crate) async fn liquidate(&mut self) {
        let owners = self.balances.indices().await.unwrap();
        for owner in owners {
            let mut amounts = match self.balances.get(&owner).await {
                Ok(Some(amounts)) => amounts,
                _ => continue,
            };
            let mut spendable = match self.spendables.get(&owner).await {
                Ok(Some(spendable)) => spendable,
                _ => continue,
            };
            amounts.amounts.retain(|amount| {
                let expired = current_system_time().saturating_diff_micros(amount.expired) > 0;
                log::info!(
                    "Current {:?} expired at {:?} expired {} amount {}",
                    current_system_time(),
                    amount.expired,
                    expired,
                    amount.amount
                );
                if expired {
                    self.balance
                        .set(self.balance.get().saturating_add(amount.amount));
                    spendable = spendable.saturating_sub(amount.amount);
                }
                !expired
            });
            self.spendables.insert(&owner, spendable).unwrap();
            self.balances.insert(&owner, amounts).unwrap();
        }
    }

    pub(crate) async fn set_callers(&mut self, application_ids: Vec<ApplicationId>) {
        application_ids
            .iter()
            .for_each(|application_id| self.callers.insert(application_id).unwrap())
    }
}

#[derive(Debug, Error)]
pub enum StateError {
    #[error("Insufficient supply balance")]
    InsufficientSupplyBalance,

    #[error("Insufficient account balance")]
    _InsufficientAccountBalance,

    #[error("View error")]
    ViewError(#[from] linera_views::views::ViewError),
}
