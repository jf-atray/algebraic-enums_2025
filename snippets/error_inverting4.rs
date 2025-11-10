fn try_listen( source: &Network, sink: &File ) -> Result<> {
  let input = source.read()
    .ok_or(ListenErrors::PipePanic)?;
  let text = input.parse()
    .ok_or(ListenErrors::MangledData)?;
  let customer = input.try_new()
    .ok_or(ListenErrors::BusinessLogic)?;
    
  Ok(customer)
}