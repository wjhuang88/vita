package zone.hwj.vita.codec.impl;

import java.io.InputStream;
import java.util.List;
import reactor.core.publisher.FluxSink;
import zone.hwj.vita.codec.BufferTranslator;
import zone.hwj.vita.def.NativeBuffer;
import zone.hwj.vita.tools.IoUtils;
import zone.hwj.vita.tools.SlicedBufferPool;

public class BytesTranslator implements BufferTranslator<byte[]> {

    @Override
    public byte[] decode(InputStream is) throws Throwable {
        return IoUtils.toByteArray(is);
    }

    @Override
    public void encode(byte[] source, FluxSink<NativeBuffer> sink) throws Throwable {
        List<NativeBuffer> bufferList = SlicedBufferPool.common().acquire(source);
        bufferList.forEach(sink::next);
    }
}
