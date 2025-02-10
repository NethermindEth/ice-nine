pub mod particle;
pub mod substance;

use crate::keeper::KeeperLink;
use crate::router::RouterLink;
use crate::space::SpaceLink;
use substance::SubstanceLink;

#[derive(Clone)]
pub struct SubstanceLinks {
    pub substance: SubstanceLink,
    pub keeper: KeeperLink,
    pub router: RouterLink,
    pub space: SpaceLink,
}
