pub trait ToolRequest: Send + 'static {
    type ToolResponse: Send + 'static;
}
