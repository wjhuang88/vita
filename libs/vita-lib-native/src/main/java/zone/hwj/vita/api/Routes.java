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
        RequestBodyHandler handler();
    }

    class Builder {
        private final MappedList<Route> routeList = new MappedList<>(Route::pattern);

        public Routes build() {
            return () -> routeList;
        }

        public Builder route(String pattern, RequestBodyHandler handler) {
            routeList.add(new Route() {
                @Override
                public String pattern() {
                    return pattern;
                }

                @Override
                public RequestBodyHandler handler() {
                    return handler;
                }
            });
            return this;
        }
    }
}
