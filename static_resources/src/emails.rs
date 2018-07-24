use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendMail {
    pub to: String,
    pub subject: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderUpdateStateForUser {
    pub user_email: String,
    pub order_slug: String,
    pub order_state: OrderState,
    pub cluster_url: String,
}

pub trait Email {
    fn into_send_mail(self) -> SendMail;
}

impl Email for OrderUpdateStateForUser {
    fn into_send_mail(self) -> SendMail {
        SendMail {
            to : self.user_email,
            subject : format!("Your order {} has changed.", self.order_slug),
            text : format!(
                "Orders' {} state is '{}' now. You can watch current info about your order on <a href=\"{}/profile/orders/{}\">this page</a>.",
                self.order_slug, self.order_state, self.cluster_url, self.order_slug
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderUpdateStateForStore {
    pub store_email: String,
    pub order_slug: String,
    pub order_state: OrderState,
    pub cluster_url: String,
    pub store_id: String,
}

impl Email for OrderUpdateStateForStore {
    fn into_send_mail(self) -> SendMail {
        SendMail {
            to: self.store_email,
            subject: format!("Changed orders' {} state.", self.order_slug),
            text: format!(
                "Orders' {} state is '{}' now. You can watch current order info on <a href=\"{}/manage/store/{}/orders/{}\">this page</a>.",
                self.order_slug, self.order_state, self.cluster_url, self.store_id, self.order_slug
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailVerificationForUser {
    pub user_email: String,
    pub verify_email_path: String,
    pub token: String,
}

impl Email for EmailVerificationForUser {
    fn into_send_mail(self) -> SendMail {
        SendMail {
            to: self.user_email,
            subject: "Email verification".to_string(),
            text: format!("{}/{}", self.verify_email_path, self.token),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PasswordResetForUser {
    pub user_email: String,
    pub reset_password_path: String,
    pub token: String,
}

impl Email for PasswordResetForUser {
    fn into_send_mail(self) -> SendMail {
        SendMail {
            to: self.user_email,
            subject: "Password reset".to_string(),
            text: format!("{}/{}", self.reset_password_path, self.token),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplyPasswordResetForUser {
    pub user_email: String,
}

impl Email for ApplyPasswordResetForUser {
    fn into_send_mail(self) -> SendMail {
        SendMail {
            to: self.user_email,
            subject: "Password reset success".to_string(),
            text: "Password for linked account has been successfully reset.".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplyEmailVerificationForUser {
    pub user_email: String,
}

impl Email for ApplyEmailVerificationForUser {
    fn into_send_mail(self) -> SendMail {
        SendMail {
            to: self.user_email,
            subject: "Email verification".to_string(),
            text: "Email for linked account has been verified.".to_string(),
        }
    }
}
