package zone.hwj.vita.codec;

import reactor.core.publisher.FluxSink;
import zone.hwj.vita.def.NativeBuffer;

public interface BufferEncoder<T> {
    void encode(T source, FluxSink<NativeBuffer> sink) throws Throwable;
}
