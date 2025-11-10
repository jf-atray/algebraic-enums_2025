enum CardPlay {
  Pass,
  Stand,
  Play( Card )
}

fn apply( board: &mut Boardgame, play: CardPlay ) {
  match play {    
    CardPlay::Pass => { },
    CardPlay::Stand => board.stand(),
    CardPlay::Play( card ) if board.allows( &play ) 
      => board.present( play ),
  }
}