#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimpleMail {
    pub to: String,
    pub subject: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

pub trait Email {
    fn into_send_mail(self) -> SimpleMail;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderUpdateStateForUser {
    pub user: EmailUser,
    pub order_slug: String,
    pub order_state: String,
    pub cluster_url: String,
}

impl Email for OrderUpdateStateForUser {
    fn into_send_mail(self) -> SimpleMail {
        SimpleMail {
            to : self.user.email,
            subject : format!("The order {} status", self.order_slug),
            text : format!(
                "Orders' {} state is '{}' now. You can view current info about your order on <a href=\"{}/profile/orders/{}\">this page</a>.",
                self.order_slug, self.order_state, self.cluster_url, self.order_slug
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderUpdateStateForStore {
    pub store_email: String,
    pub order_slug: String,
    pub order_state: String,
    pub cluster_url: String,
    pub store_id: String,
}

impl Email for OrderUpdateStateForStore {
    fn into_send_mail(self) -> SimpleMail {
        SimpleMail {
            to: self.store_email,
            subject: format!("The order {} status", self.order_slug),
            text: format!(
                "Orders' {} state is '{}' now. You can view current order info on <a href=\"{}/manage/store/{}/orders/{}\">this page</a>.",
                self.order_slug, self.order_state, self.cluster_url, self.store_id, self.order_slug
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderCreateForUser {
    pub user: EmailUser,
    pub order_slug: String,
    pub cluster_url: String,
}

impl Email for OrderCreateForUser {
    fn into_send_mail(self) -> SimpleMail {
        SimpleMail {
            to: self.user.email,
            subject: format!("New order {}.", self.order_slug),
            text: format!(
                "Order {} was created. You can view current info about your order on <a href=\"{}/profile/orders/{}\">this page</a>.",
                self.order_slug, self.cluster_url, self.order_slug
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderCreateForStore {
    pub store_email: String,
    pub order_slug: String,
    pub cluster_url: String,
    pub store_id: String,
}

impl Email for OrderCreateForStore {
    fn into_send_mail(self) -> SimpleMail {
        SimpleMail {
            to: self.store_email,
            subject: format!("New order {}.", self.order_slug),
            text: format!(
                "Order {} was created. You can view current order info on <a href=\"{}/manage/store/{}/orders/{}\">this page</a>.",
                self.order_slug, self.cluster_url, self.store_id, self.order_slug
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailVerificationForUser {
    pub user: EmailUser,
    pub verify_email_path: String,
    pub token: String,
}

impl Email for EmailVerificationForUser {
    fn into_send_mail(self) -> SimpleMail {
        SimpleMail {
            to: self.user.email,
            subject: "Verify your account on Storiqa".to_string(),
            text: format!("{}/{}", self.verify_email_path, self.token),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PasswordResetForUser {
    pub user: EmailUser,
    pub reset_password_path: String,
    pub token: String,
}

impl Email for PasswordResetForUser {
    fn into_send_mail(self) -> SimpleMail {
        SimpleMail {
            to: self.user.email,
            subject: "Password reset".to_string(),
            text: format!("{}/{}", self.reset_password_path, self.token),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplyPasswordResetForUser {
    pub user: EmailUser,
}

impl Email for ApplyPasswordResetForUser {
    fn into_send_mail(self) -> SimpleMail {
        SimpleMail {
            to: self.user.email,
            subject: "Successful password reset".to_string(),
            text: "Password for linked account has been successfully reset.".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplyEmailVerificationForUser {
    pub user: EmailUser,
}

impl Email for ApplyEmailVerificationForUser {
    fn into_send_mail(self) -> SimpleMail {
        SimpleMail {
            to: self.user.email,
            subject: "Successful registration".to_string(),
            text: "Email for linked account has been verified.".to_string(),
        }
    }
}

#[derive(GraphQLEnum, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Debug, DieselTypes)]
#[graphql(name = "TemplateVariant", description = "Template variant")]
pub enum TemplateVariant {
    #[graphql(description = "email template for order update state for user.")]
    OrderUpdateStateForUser,
    #[graphql(description = "email template for order update state for store.")]
    OrderUpdateStateForStore,
    #[graphql(description = "email template for oder create for user.")]
    OrderCreateForUser,
    #[graphql(description = "email template for oder create for store.")]
    OrderCreateForStore,
    #[graphql(description = "email template for email verification.")]
    EmailVerificationForUser,
    #[graphql(description = "email template for password reset.")]
    PasswordResetForUser,
    #[graphql(description = "email template for apply password reset.")]
    ApplyPasswordResetForUser,
    #[graphql(description = "email template for apply email verification.")]
    ApplyEmailVerificationForUser,
}
