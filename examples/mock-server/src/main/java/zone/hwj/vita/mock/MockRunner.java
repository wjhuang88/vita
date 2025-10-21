package zone.hwj.vita.mock;

import java.util.Map;
import zone.hwj.vita.Vita;
import zone.hwj.vita.api.RequestHandler;
import zone.hwj.vita.api.Routes;
import zone.hwj.vita.codec.BufferTranslator;
import zone.hwj.vita.codec.impl.JsonTranslator;
import zone.hwj.vita.codec.impl.StringTranslator;

public class MockRunner {
    public static void main(String[] args) {

        Vita manager = Vita.getInstance();
        BufferTranslator<String> strTranslator = new StringTranslator();
        BufferTranslator<Object> jsonTranslator = new JsonTranslator();
        Routes routes = Routes.builder()
                .route("/123", RequestHandler.make(strTranslator, strTranslator, req -> (req + " form java!\n").repeat(3)))
                .route("/test", RequestHandler.make(jsonTranslator, jsonTranslator, req -> {
                    System.out.println(req);
                    return Map.of("testKey", "testValue", "testKey2", 123);
                }))
                .build();
        manager.startServer(routes);
    }
}
