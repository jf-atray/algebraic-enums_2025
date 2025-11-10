fn top_loop( &mut self, rx: Receiver<ContextMessage> ) {
    'life: loop {
        for job in rx {
            
            match job.payload {

                MessagePayload::IntegratePhysics(bucket, span) => {
                    let mem = self.physics.get_mut(bucket)[ span ];
                    Self::Simd_integrate( mem )
                        .expect("No support for physics failures, must panic.");
                },

                MessagePayload::ProcessAI(id) => {
                    let brains = self.ai.get_mut(id);
                    let concept = self.ai.back_ref(brains.0);

                    brains.think_for( &concept );
                },

                MessagePayload::CloseFrame => break 'life;

            }

        }
        thread::yield();
    }
}