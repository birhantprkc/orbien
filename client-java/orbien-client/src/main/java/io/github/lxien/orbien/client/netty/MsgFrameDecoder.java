package io.github.lxien.orbien.client.netty;

import io.github.lxien.orbien.client.msg.MsgCodec;
import io.github.lxien.orbien.client.msg.WireMessage;
import io.netty.buffer.ByteBuf;
import io.netty.channel.ChannelHandlerContext;
import io.netty.handler.codec.ByteToMessageDecoder;

import java.util.List;

/**
 * Wire frame: {@code type(u8) + length(u32 LE) + body}
 */
public final class MsgFrameDecoder extends ByteToMessageDecoder {

    @Override
    protected void decode(ChannelHandlerContext ctx, ByteBuf in, List<Object> out) throws Exception {
        if (in.readableBytes() < 5) {
            return;
        }
        in.markReaderIndex();
        byte type = in.readByte();
        long len = in.readUnsignedIntLE();
        if (len > MsgCodec.MAX_BODY) {
            throw new IllegalStateException("message too large: " + len);
        }
        if (in.readableBytes() < len) {
            in.resetReaderIndex();
            return;
        }
        byte[] body = new byte[(int) len];
        in.readBytes(body);
        WireMessage msg = MsgCodec.decode(type, body);
        out.add(msg);
    }
}
