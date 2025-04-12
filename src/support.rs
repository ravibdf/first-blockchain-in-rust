 pub struct Block<Header, Extrinsic> {
    pub extrinsics: Vec<Extrinsic>,
 }

 pub struct Header<BlockNumber> {
    pub block_number: BlockNumber,
 }

 pub struct Extrinsic<Caller, Call> {
    pub caller: Caller,
    pub call: Call,
 }

 pub type DispatchResult = Result<(), &'static str>;

 pub trait Dispatch {
    type caller;

    type call;

    fn dispatch(&mut self, caller: Self::caller, call: Self::call) -> DispatchResult;
 }