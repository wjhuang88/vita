package zone.hwj.vita.api;

import java.io.InputStream;
import java.util.function.BiConsumer;
import java.util.function.Function;
import reactor.core.publisher.FluxSink;
import zone.hwj.vita.codec.BufferDecoder;
import zone.hwj.vita.codec.BufferEncoder;
import zone.hwj.vita.def.NativeBuffer;

public interface RequestHandler<REQ, RES> extends BiConsumer<InputStream, FluxSink<NativeBuffer>> {

    static <REQ, RES> RequestHandler<REQ, RES> make(BufferEncoder<RES> encoder, BufferDecoder<REQ> decoder, Function<REQ, RES> handler) {
        return new RequestHandler<>() {
            @Override
            public RES handle(REQ req) {
                return handler.apply(req);
            }

            @Override
            public BufferDecoder<REQ> decoder() {
                return decoder;
            }

            @Override
            public BufferEncoder<RES> encoder() {
                return encoder;
            }
        };
    }

    RES handle(REQ req);

    BufferDecoder<REQ> decoder();

    BufferEncoder<RES> encoder();

    @Override
    default void accept(InputStream is, FluxSink<NativeBuffer> sink) {
        try {
            REQ decoded = decoder().decode(is);
            encoder().encode(handle(decoded), sink);
        } catch (Throwable e) {
            sink.error(e);
            return;
        }
        sink.complete();
    }
}
