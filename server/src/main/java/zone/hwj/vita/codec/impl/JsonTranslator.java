package zone.hwj.vita.codec.impl;

import com.alibaba.fastjson2.JSON;
import com.alibaba.fastjson2.JSONFactory;
import java.io.InputStream;
import reactor.core.publisher.FluxSink;
import zone.hwj.vita.codec.BufferTranslator;
import zone.hwj.vita.def.NativeBuffer;
import zone.hwj.vita.tools.NativeBufferOutputStream;
import zone.hwj.vita.tools.SlicedBufferPool;

public class JsonTranslator implements BufferTranslator<Object> {

    private final SlicedBufferPool bufferPool = SlicedBufferPool.common();

    @Override
    public Object decode(InputStream is) throws Throwable {
        return JSON.parse(is, JSONFactory.createReadContext());
    }

    @Override
    public void encode(Object source, FluxSink<NativeBuffer> sink) throws Throwable {
        try(NativeBufferOutputStream os = new NativeBufferOutputStream(bufferPool, sink)) {
            JSON.writeTo(os, source);
        }
    }
}
