use bytes::BytesMut;

#[derive(Debug, Clone, Default)]
pub(super) struct _Frame {
    // 1位，用于描述消息是否结束，如果位1表示该消息为消息尾部，如果为0则还有后续数据包。
    pub(super) _fin: bool,
    // RSV1,RSV2,RSV3:各1位，用于扩展定义，没有扩展约定则必须为0
    pub(super) _rsv1: bool,
    pub(super) _rsv2: bool,
    pub(super) _rsv3: bool,
    // 4位，最多表示15种类型消息
    // OPCODE的定义范围：
    // %x0表示连续消息片断
    // %x1表示文本消息片断
    // %x2表示二进制消息片断
    // %x8表示连接关闭
    // %x9表示心跳检查ping
    // %xa表示心跳检查pong
    pub(super) _opcode: u8,
    // 1位，用于表示PayloadData是否经过掩码处理，客户端发出的数据帧需要进行掩码处理，所以此位1。
    pub(super) _mask: bool,
    // Payload length===x，如果
    //          如果x值在0-125，则payload即为真实长度。
    //         如果x值是126，则后面2个字节形成的16位无符号整型数的值是payload的真实长度。
    //          如果x值是127，则后面8个字节形成的64位无符号整型数的值是payload的真实长度。
    pub(super) _payload_len: u64,
    pub(super) _mask_key: [u8; 4],
    pub(super) _data: BytesMut,
}
