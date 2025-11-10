struct SourceId;
struct ContextMessage {
    id: SourceId,
    payload: MessagePayload,
}

enum MessagePayload(
    IntegratePhysics {
        bucket: usize,
        span: Range<usize>,
    },
    ProcessAI( NpcId ),
    CloseFrame,
)
