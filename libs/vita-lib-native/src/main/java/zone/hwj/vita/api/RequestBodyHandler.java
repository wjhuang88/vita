package zone.hwj.vita.api;

import java.io.InputStream;
import java.util.function.BiConsumer;
import reactor.core.publisher.FluxSink;
import zone.hwj.vita.def.NativeBuffer;

public interface RequestBodyHandler extends BiConsumer<InputStream, FluxSink<NativeBuffer>> {

}
