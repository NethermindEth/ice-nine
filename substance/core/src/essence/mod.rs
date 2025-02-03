pub mod particle;
pub mod substance;

use crate::keeper::KeeperLink;
use crate::router::RouterLink;
use crate::space::SpaceLink;

#[derive(Clone)]
pub struct SubstanceLinks {
    pub keeper: KeeperLink,
    pub router: RouterLink,
    pub space: SpaceLink,
}
