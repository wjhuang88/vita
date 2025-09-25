package zone.hwj.vita.tools.pair;

import java.util.HashMap;
import java.util.Map;

public interface Pair<L, R> extends Map.Entry<L, R> {

    static <L, R> Pair<L, R> of(L left, R right) {
        return new ImmutablePair<>(left, right);
    }

    L getLeft();

    R getRight();

    default Map<L, R> asMap() {
        final HashMap<L, R> map = new HashMap<>();
        map.put(getLeft(), getRight());
        return map;
    }
}
