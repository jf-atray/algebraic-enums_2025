enum Transaction {
    Dispose( Vec<Specimen> ),
    Sort( Rack, Vec<Specimen> )
    Audit
}

struct Specimen;
struct Rack;