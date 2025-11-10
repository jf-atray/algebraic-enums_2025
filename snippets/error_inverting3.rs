fn try_listen( source: &Network, sink: &File ) -> Result<> {
  let Some(input) = source.read() else {
    return Err( ListenErrors::PipePanic );
  };
  let Some(text) = input.parse() else {
    return Err( ListenErrors::MangledData );
  };
  let Some(customer) = input.try_new() else {
    return Err( ListenErrors::BusinessLogic );
  };

  Ok(customer)
}