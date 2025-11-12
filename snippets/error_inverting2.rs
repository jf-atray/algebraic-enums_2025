fn try_listen( source: &Network, sink: &File ) -> Result<Customer, ListenErrors> {
  let Option::Some(input) = source.read() else {
    return Result::Err( ListenErrors::PipePanic );
  };
  let Option::Some(text) = input.parse() else {
    return Result::Err( ListenErrors::MangledData );
  };
  let Option::Some(customer) = input.try_new() else {
    return Result::Err( ListenErrors::BusinessLogic );
  };

  Result::Ok(customer)
}