#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::collections::HashMap;

use self::state::Feed;
use credit::CreditAbi;
use feed::{
    Content, FeedError, FeedParameters, FeedResponse, InstantiationArgument, Message, Operation,
};
use foundation::FoundationAbi;
use linera_sdk::{
    base::{Amount, ApplicationId, ChannelName, Destination, MessageId, Owner, WithContractAbi},
    views::{RootView, View},
    Contract, ContractRuntime,
};

pub struct FeedContract {
    state: Feed,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(FeedContract);

impl WithContractAbi for FeedContract {
    type Abi = feed::FeedAbi;
}

const SUBSCRIPTION_CHANNEL: &[u8] = b"subscriptions";

impl Contract for FeedContract {
    type Message = Message;
    type InstantiationArgument = InstantiationArgument;
    type Parameters = FeedParameters;

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = Feed::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        FeedContract { state, runtime }
    }

    async fn instantiate(&mut self, state: Self::InstantiationArgument) {
        self.runtime.application_parameters();
        self.state.instantiate_feed(state).await;
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            Operation::Like { cid } => self.on_op_like(cid).expect("Failed OP: like"),
            Operation::Dislike { cid } => self.on_op_dislike(cid).expect("Failed OP: dislike"),
            Operation::Tip { cid, amount } => self.on_op_tip(cid, amount).expect("Failed OP: tip"),
            Operation::RequestSubscribe => self
                .on_op_request_subscribe()
                .expect("Failed OP: subscribe"),
            Operation::Recommend {
                cid,
                reason_cid,
                reason,
            } => self
                .on_op_recommend(cid, reason_cid, reason)
                .expect("Failed OP: recommend"),
            Operation::Comment {
                cid,
                comment_cid,
                comment,
                commentor,
            } => self
                .on_op_comment(cid, comment_cid, comment, commentor)
                .expect("Failed OP: comment"),
            Operation::Publish {
                cid,
                title,
                content,
                author,
                cover,
                abbreviation,
            } => self
                .on_op_publish(cid, title, content, author, cover, abbreviation)
                .expect("Failed OP: publish"),
            Operation::ContentAuthor { cid } => self
                .on_op_content_author(cid)
                .await
                .expect("Failed OP: content author"),
        }
    }

    async fn execute_message(&mut self, message: Self::Message) {
        match message {
            Message::Like { cid } => self.on_msg_like(cid).await.expect("Failed MSG: like"),
            Message::Dislike { cid } => {
                self.on_msg_dislike(cid).await.expect("Failed MSG: dislike")
            }
            Message::Tip { cid, amount } => {
                self.on_msg_tip(cid, amount).await.expect("Failed MSG: tip")
            }
            Message::Publish {
                cid,
                title,
                content,
                author,
                cover,
                abbreviation,
            } => self
                .on_msg_publish(cid, title, content, author, cover, abbreviation)
                .await
                .expect("Failed MSG: publish"),
            Message::Recommend {
                cid,
                reason_cid,
                reason,
            } => self
                .on_msg_recommend(cid, reason_cid, reason)
                .await
                .expect("Failed MSG: recommend"),
            Message::Comment {
                cid,
                comment_cid,
                comment,
                commentor,
            } => self
                .on_msg_comment(cid, comment_cid, comment, commentor)
                .await
                .expect("Failed MSG: comment"),
            Message::RequestSubscribe => self
                .on_msg_request_subscribe()
                .expect("Failed MSG: subscribe"),
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl FeedContract {
    fn credit_app_id(&mut self) -> ApplicationId<CreditAbi> {
        self.runtime.application_parameters().credit_app_id
    }

    fn foundation_app_id(&mut self) -> ApplicationId<FoundationAbi> {
        self.runtime.application_parameters().foundation_app_id
    }

    async fn reward_credits(&mut self, owner: Owner, amount: Amount) -> Result<(), FeedError> {
        let call = credit::Operation::Reward { owner, amount };
        let credit_app_id = self.credit_app_id();
        let _ = self.runtime.call_application(true, credit_app_id, &call);
        Ok(())
    }

    async fn reward_tokens(&mut self, author: Owner) -> Result<(), FeedError> {
        let call = foundation::Operation::Reward {
            reward_user: Some(author),
            reward_type: foundation::RewardType::Publish,
            activity_id: None,
        };
        let foundation_app_id = self.foundation_app_id();
        let _ = self
            .runtime
            .call_application(true, foundation_app_id, &call);
        Ok(())
    }

    async fn publish(
        &mut self,
        cid: String,
        comment_to_cid: Option<String>,
        title: String,
        content: String,
        author: Owner,
        cover: String,
        abbreviation: String,
        creation_chain: bool,
    ) -> Result<(), FeedError> {
        match self
            .state
            .create_content(
                Content {
                    cid,
                    comment_to_cid,
                    title,
                    content,
                    cover,
                    abbreviation,
                    author,
                    likes: 0,
                    dislikes: 0,
                    accounts: HashMap::default(),
                    created_at: self.runtime.system_time(),
                },
                author,
            )
            .await
        {
            Ok(_) => {
                if !creation_chain {
                    return Ok(());
                }
                self.reward_credits(author, Amount::from_tokens(500))
                    .await?;
                self.reward_tokens(author).await?;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    async fn like(
        &mut self,
        cid: String,
        owner: Owner,
        creation_chain: bool,
    ) -> Result<(), FeedError> {
        match self
            .state
            .like_content(cid, owner, true, self.runtime.system_time())
            .await
        {
            Ok(_) => {
                if !creation_chain {
                    return Ok(());
                }
                return self.reward_credits(owner, Amount::from_tokens(100)).await;
            }
            Err(err) => Err(err),
        }
    }

    async fn dislike(
        &mut self,
        cid: String,
        owner: Owner,
        creation_chain: bool,
    ) -> Result<(), FeedError> {
        match self
            .state
            .like_content(cid, owner, false, self.runtime.system_time())
            .await
        {
            Ok(_) => {
                if !creation_chain {
                    return Ok(());
                }
                return self.reward_credits(owner, Amount::from_tokens(100)).await;
            }
            Err(err) => Err(err),
        }
    }

    fn require_message_id(&mut self) -> Result<MessageId, FeedError> {
        match self.runtime.message_id() {
            Some(message_id) => Ok(message_id),
            None => Err(FeedError::InvalidMessageId),
        }
    }

    fn require_authenticated_signer(&mut self) -> Result<Owner, FeedError> {
        match self.runtime.authenticated_signer() {
            Some(owner) => Ok(owner),
            None => Err(FeedError::InvalidSigner),
        }
    }

    fn on_op_like(&mut self, cid: String) -> Result<FeedResponse, FeedError> {
        self.runtime
            .prepare_message(Message::Like { cid })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(FeedResponse::Ok)
    }

    fn on_op_dislike(&mut self, cid: String) -> Result<FeedResponse, FeedError> {
        self.runtime
            .prepare_message(Message::Dislike { cid })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(FeedResponse::Ok)
    }

    fn on_op_tip(&mut self, cid: String, amount: Amount) -> Result<FeedResponse, FeedError> {
        self.runtime
            .prepare_message(Message::Tip { cid, amount })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(FeedResponse::Ok)
    }

    fn on_op_request_subscribe(&mut self) -> Result<FeedResponse, FeedError> {
        self.runtime
            .prepare_message(Message::RequestSubscribe)
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(FeedResponse::Ok)
    }

    fn on_op_recommend(
        &mut self,
        cid: String,
        reason_cid: String,
        reason: String,
    ) -> Result<FeedResponse, FeedError> {
        self.runtime
            .prepare_message(Message::Recommend {
                cid,
                reason_cid,
                reason,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(FeedResponse::Ok)
    }

    fn on_op_comment(
        &mut self,
        cid: String,
        comment_cid: String,
        comment: String,
        commentor: Owner,
    ) -> Result<FeedResponse, FeedError> {
        self.runtime
            .prepare_message(Message::Comment {
                cid,
                comment_cid,
                comment,
                commentor,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(FeedResponse::Ok)
    }

    fn on_op_publish(
        &mut self,
        cid: String,
        title: String,
        content: String,
        author: Owner,
        cover: String,
        abbreviation: String,
    ) -> Result<FeedResponse, FeedError> {
        self.runtime
            .prepare_message(Message::Publish {
                cid,
                title,
                content,
                cover,
                abbreviation,
                author,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(FeedResponse::Ok)
    }

    async fn on_op_content_author(&mut self, cid: String) -> Result<FeedResponse, FeedError> {
        match self.state.content_author(cid).await {
            Ok(owner) => Ok(FeedResponse::ContentAuthor(Some(owner))),
            _ => Err(FeedError::InvalidContent),
        }
    }

    async fn on_msg_like(&mut self, cid: String) -> Result<(), FeedError> {
        let signer = self.require_authenticated_signer()?;
        let creation_chain =
            self.runtime.chain_id() == self.runtime.application_id().creation.chain_id;
        self.like(cid.clone(), signer, creation_chain).await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Like { cid })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_dislike(&mut self, cid: String) -> Result<(), FeedError> {
        let signer = self.require_authenticated_signer()?;
        let creation_chain =
            self.runtime.chain_id() == self.runtime.application_id().creation.chain_id;
        self.dislike(cid.clone(), signer, creation_chain).await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Dislike { cid })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_tip(&mut self, cid: String, amount: Amount) -> Result<(), FeedError> {
        // TODO: transfer amount from signer to author
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Tip { cid, amount })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_publish(
        &mut self,
        cid: String,
        title: String,
        content: String,
        author: Owner,
        cover: String,
        abbreviation: String,
    ) -> Result<(), FeedError> {
        let creation_chain =
            self.runtime.chain_id() == self.runtime.application_id().creation.chain_id;
        self.publish(
            cid.clone(),
            None,
            title.clone(),
            content.clone(),
            author,
            cover.clone(),
            abbreviation.clone(),
            creation_chain,
        )
        .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Publish {
                cid,
                title,
                content,
                cover,
                abbreviation,
                author,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_recommend(
        &mut self,
        cid: String,
        reason_cid: String,
        reason: String,
    ) -> Result<(), FeedError> {
        let author = self.require_authenticated_signer()?;
        let creation_chain =
            self.runtime.chain_id() == self.runtime.application_id().creation.chain_id;
        self.publish(
            reason_cid.clone(),
            Some(cid.clone()),
            String::default(),
            reason.clone(),
            author,
            "".to_string(),
            "".to_string(),
            creation_chain,
        )
        .await?;
        self.state
            .recommend_content(cid.clone(), reason_cid.clone())
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Recommend {
                cid,
                reason_cid,
                reason,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_comment(
        &mut self,
        cid: String,
        comment_cid: String,
        comment: String,
        commentor: Owner,
    ) -> Result<(), FeedError> {
        let creation_chain =
            self.runtime.chain_id() == self.runtime.application_id().creation.chain_id;
        self.publish(
            comment_cid.clone(),
            Some(cid.clone()),
            String::default(),
            comment.clone(),
            commentor,
            "".to_string(),
            "".to_string(),
            creation_chain,
        )
        .await?;
        self.state
            .comment_content(cid.clone(), comment_cid.clone())
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::Comment {
                cid,
                comment_cid,
                comment,
                commentor,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    fn on_msg_request_subscribe(&mut self) -> Result<(), FeedError> {
        let message_id = self.require_message_id()?;
        // The subscribe message must be from another chain
        if message_id.chain_id == self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        self.runtime.subscribe(
            message_id.chain_id,
            ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()),
        );
        Ok(())
    }
}
