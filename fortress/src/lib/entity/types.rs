pub enum EntityType {
    PLAYER,
    GROUND,
}

/*
impl <T> BeginContact for Interact<PlayerFoot, T> {
    pub fn begin_contact(&self) {
        let player: &mut world::Player = self.first.resolve();
    }
}

match (e1.etype(), e2.etype()) {
    (PlayerEntity(PlayerFoot), _) => {
        Interact::new(PlayerFoot, (), e1, e2).begin_contact();
    }
}
*/