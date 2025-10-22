package zone.hwj.vita.mock;

import java.nio.charset.StandardCharsets;
import java.util.List;
import java.util.Map;
import zone.hwj.vita.Vita;
import zone.hwj.vita.api.RequestHandler;
import zone.hwj.vita.api.Routes;
import zone.hwj.vita.codec.BufferEncoder;
import zone.hwj.vita.codec.BufferTranslator;
import zone.hwj.vita.codec.impl.JsonTranslator;
import zone.hwj.vita.codec.impl.StringTranslator;
import zone.hwj.vita.def.NativeBuffer;
import zone.hwj.vita.tools.SlicedBufferPool;

public class MockRunner {
    public static void main(String[] args) {

        Vita manager = Vita.getInstance();
        BufferTranslator<String> strTranslator = new StringTranslator();
        BufferTranslator<Object> jsonTranslator = new JsonTranslator();
        BufferEncoder<String> customEncoder = (source, sink) -> {
            for (int i = 0; i < 4; i++) {
                String r = i + source;
                List<NativeBuffer> bufferList = SlicedBufferPool.common()
                        .acquire(r.getBytes(StandardCharsets.UTF_8));
                bufferList.forEach(sink::next);
                Thread.sleep(1000);
            }
        };
        Routes routes = Routes.builder()
                .route("/123", RequestHandler.make(strTranslator, strTranslator, req -> (req + " form java!\n").repeat(3)))
                .route("/stream", RequestHandler.make(customEncoder, strTranslator, req -> ": " + req + "\n"))
                .route("/test/{dy}", RequestHandler.make(jsonTranslator, jsonTranslator, req -> {
                    System.out.println(req);
                    return Map.of("testKey", "testValue", "testKey2", 123);
                }))
                .build();
        manager.startServer(routes);
    }
}
