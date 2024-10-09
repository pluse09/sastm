use shaku::module;
use shaku::{HasComponent};

use crate::infrastructure::repository::project_repository::ProjectRepositoryImpl;
use crate::infrastructure::repository::mock::project_repository::ProjectRepositoryMock;

use crate::infrastructure::repository::slack_notification_repository::SlackNotificationRepositoryImpl;
use crate::infrastructure::repository::mock::slack_notification_repository::SlackNotificationRepositoryMock;

use crate::infrastructure::handler::sast_command_handler::SastCommandHandlerImpl;
use crate::infrastructure::handler::mock::sast_command_handler::SastCommandHandlerMock;


// todo: 置く場所考える
module! {
    pub ImplModule {
        components = [ProjectRepositoryImpl, SlackNotificationRepositoryImpl, SastCommandHandlerImpl],
        providers = []
    }
}

module! {
    pub MockModule {
        components = [ProjectRepositoryMock, SlackNotificationRepositoryMock, SastCommandHandlerMock],
        providers = []
    }
}