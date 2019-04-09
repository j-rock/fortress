use enum_iterator::IntoEnumIterator;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Copy, Clone, Debug, Deserialize, IntoEnumIterator)]
pub enum Buff {
    MoveSpeed,
    NumJumps,
    JumpStrength,

    SwordAttack,
    SwordKnockback,
    SwordSlashSpeed,

    CrossbowAttack,
    CrossbowFiringSpeed,
    ArrowSpeed,
    ArrowKnockback,
}

impl Distribution<Buff> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Buff {
        let all_buffs: Vec<Buff> = Buff::into_enum_iter().collect();
        all_buffs[rng.gen_range(0, all_buffs.len())]
    }
}
