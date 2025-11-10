// Custom cards come out every week..
// A valid card all code needs to handle
// is a fn pointer to arbitrary code!
enum Behavior {
  Number,
  TieBreaker,
  PlusMinus( bool ),
  // name these fields explicitly.
  Flip {
    second: i8,
  },
  // dynamic dispatch to somewhere unknown on the heap..
  Complex( Box<dyn Fn(&mut PazaakBoard, &PazaakCard)> )
}

// no matter the behavior, all cards
// have a value, 1 to 10
struct PazaakCard {
  value: i8,
  behavior: Behavior
}
struct PazaakBoard;