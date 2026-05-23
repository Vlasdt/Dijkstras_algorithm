# Задача сетевого планирования (критический путь)

## Постановка задачи

Дан проект, состоящий из `n` этапов (вершин). Заданы отношения предшествования: для каждого этапа `j` известно множество этапов `Γ⁺(j)`, которые должны быть завершены до начала `j`. Каждая дуга `(i, j)` имеет вес `tᵢⱼ` – минимальная задержка (в днях) между началом этапа `i` и началом этапа `j`. Сетевой график – ориентированный ациклический граф (DAG).

Необходимо найти **критический путь** – самый длинный путь от источника (вершина с нулевой полустепенью захода) до стока (вершина с нулевой полустепенью исхода). Длина критического пути определяет минимальное время выполнения проекта, а сам путь выделяет этапы, задержка которых недопустима.

## Решение

Для нахождения критического пути используется **алгоритм Флойда**, для поиска максимальных расстояний. Алгоритм Дейкстры не подходит, так как:
- Он ищет кратчайшие пути, а не самые длинные.
- Попытка найти максимум с помощью жадной стратегии не гарантирует оптимальность (даже при положительных весах).
- В случае смены знаков весов (для сведения к кратчайшему пути) появляются отрицательные веса, с которыми Дейкстра не работает.

Алгоритм Флойда–Уоршелла за `O(n³)` находит максимальные расстояния между всеми парами вершин и позволяет восстановить путь с помощью матрицы `next`.

---

## Псевдокод алгоритмов

### 1. Алгоритм Дейкстры (кратчайшие пути от источника, веса ≥ 0)
function Dijkstra(Graph, source):
// Инициализация
for each vertex v:
dist[v] = INF
prev[v] = undefined
visited[v] = false

dist[source] = 0

while exists vertex with visited[v] == false:
// Выбор необработанной вершины с минимальным dist
u = vertex with min dist among unvisited
visited[u] = true

for each neighbour v of u:
if not visited[v]:
alt = dist[u] + weight(u, v)
if alt < dist[v]:
dist[v] = alt
prev[v] = u

return dist, prev
**Восстановление пути** от `source` до `target`:path = []
v = target
while v != source:
path.prepend(v)
v = prev[v]
path.prepend(source)

---

### 2. Алгоритм Флойда–Уоршелла (кратчайшие пути между всеми парами)
function FloydWarshall(Graph, n):
// Инициализация матриц
let dist[n][n], next[n][n]

for i = 0 to n-1:
for j = 0 to n-1:
if i == j:
dist[i][j] = 0
next[i][j] = i
else if edge (i -> j) exists:
dist[i][j] = weight(i, j)
next[i][j] = j
else:
dist[i][j] = INF
next[i][j] = null

// Основной цикл
for k = 0 to n-1:
for i = 0 to n-1:
for j = 0 to n-1:
if dist[i][k] != INF and dist[k][j] != INF:
newDist = dist[i][k] + dist[k][j]
if newDist < dist[i][j]:
dist[i][j] = newDist
next[i][j] = next[i][k]

return dist, next

**Восстановление пути** от `i` до `j`:
if next[i][j] == null:
return [] // пути нет
path = [i]
cur = i
while cur != j:
cur = next[cur][j]
path.append(cur)
return path

---

### 3. Модификация для поиска самых длинных путей (критический путь)

Заменить:
- `INF` на `-INF` (очень маленькое число)
- `newDist < dist[i][j]` на `newDist > dist[i][j]`
- Вместо `dist[i][j] = newDist` – присваивание максимума
function CriticalPath(Graph, n):
// Найти источник s (in_degree = 0) и сток t (out_degree = 0)

dist[i][j] = -INF
next[i][j] = null
for i = 0 to n-1:
dist[i][i] = 0
next[i][i] = i

for each edge (u, v, w):
if w > dist[u][v]:
dist[u][v] = w
next[u][v] = v

for k = 0 to n-1:
for i = 0 to n-1:
if dist[i][k] == -INF: continue
for j = 0 to n-1:
if dist[k][j] == -INF: continue
newDist = dist[i][k] + dist[k][j]
if newDist > dist[i][j]:
dist[i][j] = newDist
next[i][j] = next[i][k]

length = dist[s][t]
path = восстановить по next[s][t]
return length, path

---

## Применение к задаче

Для сетевого графика (14 вершин, 25 дуг) алгоритм Флойда–Уоршелла (версия для максимума) находит:
- Длину критического пути (в днях)
- Последовательность критических этапов

Сложность `O(14³) = 2744` операций – выполняется мгновенно. Результат позволяет определить минимальное время реализации проекта и выявить наиболее ответственные этапы.
