package zone.hwj.vita.codec;

import java.io.InputStream;

public interface BufferDecoder<T> {
    T decode(InputStream is) throws Throwable;
}
