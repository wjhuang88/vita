package zone.hwj.vita.codec.impl;

import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.util.List;
import reactor.core.publisher.FluxSink;
import zone.hwj.vita.codec.BufferTranslator;
import zone.hwj.vita.def.NativeBuffer;
import zone.hwj.vita.tools.IoUtils;
import zone.hwj.vita.tools.SlicedBufferPool;

public class StringTranslator implements BufferTranslator<String> {

    @Override
    public String decode(InputStream is) throws Throwable {
        return IoUtils.toUtf8String(is);
    }

    @Override
    public void encode(String source, FluxSink<NativeBuffer> sink) throws Throwable {
        List<NativeBuffer> bufferList = SlicedBufferPool.common().acquire(source.getBytes(StandardCharsets.UTF_8));
        bufferList.forEach(sink::next);
    }
}
