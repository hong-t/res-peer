use foundation::{InitialState, RewardType};
use linera_sdk::{
    base::{Amount, ArithmeticError, Owner},
    views::{MapView, RegisterView, ViewStorageContext},
};
use linera_views::views::{GraphQLView, RootView};
use thiserror::Error;

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct Foundation {
    pub foundation_balance: RegisterView<Amount>,
    pub review_reward_percent: RegisterView<u8>,
    pub review_reward_balance: RegisterView<Amount>,
    pub review_reward_factor: RegisterView<u8>,
    pub author_reward_percent: RegisterView<u8>,
    pub author_reward_balance: RegisterView<Amount>,
    pub author_reward_factor: RegisterView<u8>,
    pub activity_reward_percent: RegisterView<u8>,
    pub activity_reward_balance: RegisterView<Amount>,
    pub user_activities: MapView<Owner, Vec<u64>>,
    pub activity_lock_funds: MapView<u64, Amount>,
}

#[allow(dead_code)]
impl Foundation {
    pub(crate) async fn initialize(&mut self, state: InitialState) -> Result<(), StateError> {
        if state.review_reward_percent + state.author_reward_percent + state.activity_reward_percent
            > 100
        {
            return Err(StateError::InvalidPercent);
        }
        self.review_reward_percent.set(state.review_reward_percent);
        self.author_reward_percent.set(state.author_reward_percent);
        self.activity_reward_percent
            .set(state.activity_reward_percent);
        self.review_reward_factor.set(state.review_reward_factor);
        self.author_reward_factor.set(state.author_reward_factor);
        Ok(())
    }

    // When transaction happen, transaction fee will be deposited here
    // It'll be separated to different reward balance according to reward ratio
    pub(crate) async fn deposit(&mut self, amount: Amount) -> Result<(), StateError> {
        let review_amount = amount.try_mul(*self.review_reward_percent.get() as u128)?;
        let review_amount = review_amount.saturating_div(Amount::from_atto(100 as u128));
        let review_amount = self.review_reward_balance.get().try_add(Amount::from_atto(review_amount))?;
        
        let author_amount = amount.try_mul(*self.author_reward_percent.get() as u128)?;
        let author_amount = author_amount.saturating_div(Amount::from_atto(100 as u128));
        let author_amount = self.author_reward_balance.get().try_add(Amount::from_atto(author_amount))?;
        
        let activity_amount = amount.try_mul(*self.activity_reward_percent.get() as u128)?;
        let activity_amount = activity_amount.saturating_div(Amount::from_atto(100 as u128));
        let activity_amount = self
            .activity_reward_balance
            .get()
            .try_add(Amount::from_atto(activity_amount))?;

        self.review_reward_balance.set(review_amount);
        self.author_reward_balance.set(author_amount);
        self.activity_reward_balance.set(activity_amount);

        let _amount = amount.try_sub(review_amount)?;
        let _amount = _amount.try_sub(author_amount)?;
        let _amount = _amount.try_sub(activity_amount)?;
        let _amount = self.foundation_balance.get().try_add(_amount)?;

        self.foundation_balance.set(_amount);
        Ok(())
    }

    pub(crate) async fn reward_activity(&mut self, reward_user: Owner, amount: Amount, activity_id: u64) -> Result<(), StateError> {
        Ok(())
    }

    pub(crate) async fn reward_author(&mut self, reward_user: Owner) -> Result<(), StateError> {
        Ok(())
    }

    pub(crate) async fn reward_reviewer(&mut self, reward_user: Owner) -> Result<(), StateError> {
        Ok(())
    }

    // Reward user of different type with different balance
    pub(crate) async fn reward(
        &mut self,
        reward_user: Owner,
        reward_type: RewardType,
        amount: Option<Amount>,
        activity_id: Option<u64>,
    ) -> Result<(), StateError> {
        match reward_type {
            RewardType::Activity => self.reward_activity(reward_user, amount.unwrap(), activity_id.unwrap()).await,
            RewardType::Publish => self.reward_author(reward_user).await,
            RewardType::Review => self.reward_reviewer(reward_user).await
        }
        Ok(())
    }

    pub(crate) async fn lock(
        &mut self,
        activity_host: Owner,
        activity_id: u64,
        amount: Amount,
    ) -> Result<(), StateError> {
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum StateError {
    #[error("Invalid percent")]
    InvalidPercent,

    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("View error")]
    ViewError(#[from] linera_views::views::ViewError),

    #[error("Arithmetic error")]
    ArithmeticError(#[from] ArithmeticError),
}
