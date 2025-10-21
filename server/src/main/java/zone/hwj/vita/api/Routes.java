package zone.hwj.vita.api;

import java.util.List;
import zone.hwj.vita.tools.MappedList;

@FunctionalInterface
public interface Routes {

    static Builder builder() {
        return new Builder();
    }

    List<Route> routes();

    interface Route {
        String pattern();
        RequestHandler<?, ?> handler();
    }

    class Builder {
        private final MappedList<Route> routeList = new MappedList<>(Route::pattern);

        public Routes build() {
            return () -> routeList;
        }

        public Builder route(String pattern, RequestHandler<?, ?> handler) {
            routeList.add(new Route() {
                @Override
                public String pattern() {
                    return pattern;
                }

                @Override
                public RequestHandler<?, ?> handler() {
                    return handler;
                }
            });
            return this;
        }
    }
}
