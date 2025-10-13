package zone.hwj.vita.tools;

import java.util.ArrayList;
import java.util.Collection;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;
import java.util.function.Function;
import java.util.stream.Collectors;
import javax.annotation.Nonnull;
import zone.hwj.vita.tools.pair.Pair;

public class MappedList<T> extends ArrayList<T> {
    private final Map<String, T> map = new HashMap<>();
    private final Function<T, String> mapper;

    public MappedList(Function<T, String> mapper) {
        this.mapper = mapper;
    }

    @Override
    public boolean add(T t) {
        map.put(mapper.apply(t), t);
        return super.add(t);
    }

    @Override
    public void add(int index, T element) {
        map.put(mapper.apply(element), element);
        super.add(index, element);
    }

    @Override
    public T remove(int index) {
        T removed = super.remove(index);
        map.remove(mapper.apply(removed));
        return removed;
    }

    @Override
    public boolean remove(Object o) {
        @SuppressWarnings("unchecked")
        T casted = (T) o;
        map.remove(mapper.apply(casted));
        return super.remove(o);
    }

    @Override
    public void clear() {
        map.clear();
        super.clear();
    }

    @Override
    public boolean addAll(Collection<? extends T> c) {
        c.forEach(t -> map.put(mapper.apply(t), t));
        return super.addAll(c);
    }

    @Override
    public boolean addAll(int index, Collection<? extends T> c) {
        c.forEach(t -> map.put(mapper.apply(t), t));
        return super.addAll(index, c);
    }

    @Override
    @SuppressWarnings("unchecked")
    public boolean removeAll(Collection<?> c) {
        c.forEach(t -> map.remove(mapper.apply((T)t)));
        return super.removeAll(c);
    }

    public boolean containsKey(String key) {
        return map.containsKey(key);
    }

    public Set<String> keySet() {
        return map.keySet();
    }

    public T getItem(String key) {
        return map.get(key);
    }

    public <V> Map<String, V> asMap(Function<T, V> transformer) {
        return new MappedMap<>(transformer);
    }

    private class MappedMap<V> implements Map<String, V> {

        private final Function<T, V> transformer;

        MappedMap(Function<T, V> transformer) {
            this.transformer = transformer;
        }

        @Override
        public int size() {
            return map.size();
        }

        @Override
        public boolean isEmpty() {
            return map.isEmpty();
        }

        @Override
        public boolean containsKey(Object key) {
            return map.containsKey(key);
        }

        @Override
        public boolean containsValue(Object value) {
            @SuppressWarnings("unchecked")
            T tValue = (T) value;
            return map.containsValue(transformer.apply(tValue));
        }

        @Override
        public V get(Object key) {
            return transformer.apply(map.get(key));
        }

        @Override
        public V put(String key, V value) {
            throw new UnsupportedOperationException("This is an immutable map");
        }

        @Override
        public V remove(Object key) {
            throw new UnsupportedOperationException("This is an immutable map");
        }

        @Override
        public void putAll(@Nonnull Map<? extends String, ? extends V> m) {
            throw new UnsupportedOperationException("This is an immutable map");
        }

        @Override
        public void clear() {
            throw new UnsupportedOperationException("This is an immutable map");
        }

        @Override
        @Nonnull
        public Set<String> keySet() {
            return map.keySet();
        }

        @Override
        @Nonnull
        public Collection<V> values() {
            return map.values().stream()
                    .map(transformer)
                    .collect(Collectors.toList());
        }

        @Override
        @Nonnull
        public Set<Entry<String, V>> entrySet() {
            return map.entrySet().stream()
                    .map(entry -> Pair.of(entry.getKey(), transformer.apply(entry.getValue())))
                    .collect(Collectors.toSet());
        }
    }
}
