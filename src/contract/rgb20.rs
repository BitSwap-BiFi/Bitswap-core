use aluvm::reg::RegS;
use aluvm::data::ByteStr;
use aluvm::isa::{BytesOp, ControlFlowOp, Instr};
use rgbstd::vm::opcodes::{INSTR_PCCS, INSTR_PCVS};
use rgbstd::interface::RGB20;
use rgbstd::interface::{Amount, ContractID, Fungible, Divisible, ContractIFace, FungibleAllocation, IfaceWrapper, OutpointFilter};
use rgbstd::vm::{SubSchema, RgbIsa};
use rgb_schemata::{nia_rgb20, nia_schema};
use rgb_core::contract::commit::{Assign, TransactionType, GlobalStateType, SHA256}
use rgb_schemata::interface::{FilterIncludeAll, FungibleAllocation, IfaceClass, IssuerClass, Rgb20};

impl rgb20::interface {
    // Add your implementations here
  fn nia_rgb20() -> IfaceImpl {
    let schema = nia_schema();
    let iface = rgb20();
    let aluvm = aluvm();
  }

}
   fn IfaceImpl = {
      let version = V1
      let schema_id = schema_id();
      let iface_id = iface_id();
      let script = let script();
      let global_state = let global_state();
      let nominal = let nominal();
      let spec = let spec();
      let terms = let terms();
      let amount = amount();
      let burnepoch = let burnepoch;
      let assetowner = let assetowner;
      let created = let created();
      let issued_suply = let issued_supply();
      let entrypont = let entrypont();
      let transfer = let transfer();
}

impl rgb20::schemata {
    // Add your implementations here
    fn schemata() - IfaceImpl {
    let schema = schema();

    fn schemata = {
    let schema = let schema();
    let subschema  = let subschema();
    let alu_id  = let alu_id();
    let issue_impl = let issue_impl();
    
}

fn main() {
    println!("RGB 20 Interface");
}

