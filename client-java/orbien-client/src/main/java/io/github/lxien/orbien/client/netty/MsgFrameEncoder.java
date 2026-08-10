package io.github.lxien.orbien.client.netty;

import io.github.lxien.orbien.client.msg.MsgCodec;
import io.github.lxien.orbien.client.msg.WireMessage;
import io.netty.buffer.ByteBuf;
import io.netty.channel.ChannelHandlerContext;
import io.netty.handler.codec.MessageToByteEncoder;

public final class MsgFrameEncoder extends MessageToByteEncoder<WireMessage> {

    @Override
    protected void encode(ChannelHandlerContext ctx, WireMessage msg, ByteBuf out) {
        byte[] body = MsgCodec.encodeBody(msg.body());
        out.writeByte(msg.type());
        out.writeIntLE(body.length);
        out.writeBytes(body);
    }
}
