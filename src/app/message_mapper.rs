pub trait MessageMapper<Ms, OtherMs> {
    type SelfWithOtherMs;
    fn map_msg(self, f: impl FnOnce(Ms) -> OtherMs + 'static + Clone) -> Self::SelfWithOtherMs;
}
