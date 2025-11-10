fn into_http( &self, transaction: Transaction ) -> Option<String> {

    match transaction {

        Transaction::Dispose(specimens) => {
            let txt = Self::FormatDispose(specimens);
            Option::Some(txt)
        },

        Transaction::Sort(rack, specimens) => {
            let (rslt, txt) = Self::FormatSort(rack, specimens);
            self.update(rslt);
            Option::Some(txt)
        }

        Transaction::Audit => None

    }

}