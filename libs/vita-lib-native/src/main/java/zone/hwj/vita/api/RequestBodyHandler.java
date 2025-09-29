package zone.hwj.vita.api;

import java.io.InputStream;
import java.util.function.Function;
import reactor.core.publisher.Flux;
import zone.hwj.vita.def.NativeBuffer;

public interface RequestBodyHandler extends Function<InputStream, Flux<NativeBuffer>> {

}
