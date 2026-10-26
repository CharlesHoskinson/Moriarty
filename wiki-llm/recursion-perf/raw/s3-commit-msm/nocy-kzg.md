# Nocy KZG commitment implementation pinned historical revision
> Source: https://raw.githubusercontent.com/Nocy-io/nocy-gpu-proof-server/366d3c72e814691cd16bbd72e77b5169d07f729c/native-plonk/cuda/src/kzg.h
> Collected: 2026-10-26
> Published: Unknown

#pragma once
#include "graph\_eval.h"
#include "msm.cuh"
#include "params.h"
#include "poly.h"
#include "transcript.h"
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
namespace midnight\_plonk {
struct KzgPointsCache {
const G1Affine \*host\_ptr = nullptr;
size\_t capacity = 0;
G1Affine \*device\_ptr = nullptr;
};
struct KzgWorkCache {
size\_t capacity = 0;
Fr \*d\_scalars = nullptr;
G1Jacobian \*d\_partials = nullptr;
G1Jacobian \*d\_reduce = nullptr;
};
static thread\_local KzgPointsCache kzg\_monomial\_cache;
static thread\_local KzgPointsCache kzg\_lagrange\_cache;
static thread\_local KzgWorkCache kzg\_work\_cache;
inline void clear\_kzg\_points\_cache(KzgPointsCache &cache) {
if (cache.device\_ptr) {
cudaFree(cache.device\_ptr);
}
cache.device\_ptr = nullptr;
cache.host\_ptr = nullptr;
cache.capacity = 0;
}
inline void clear\_kzg\_work\_cache() {
if (kzg\_work\_cache.d\_scalars) {
cudaFree(kzg\_work\_cache.d\_scalars);
}
if (kzg\_work\_cache.d\_partials) {
cudaFree(kzg\_work\_cache.d\_partials);
}
if (kzg\_work\_cache.d\_reduce) {
cudaFree(kzg\_work\_cache.d\_reduce);
}
kzg\_work\_cache.d\_scalars = nullptr;
kzg\_work\_cache.d\_partials = nullptr;
kzg\_work\_cache.d\_reduce = nullptr;
kzg\_work\_cache.capacity = 0;
}
inline bool ensure\_kzg\_points\_cache(KzgPointsCache &cache, const G1Affine \*host\_ptr,
size\_t count, const char \*label, std::string &error) {
if (cache.device\_ptr && cache.host\_ptr == host\_ptr && cache.capacity >= count) {
return true;
}
clear\_kzg\_points\_cache(cache);
if (cudaMalloc(&cache.device\_ptr, count \* sizeof(G1Affine)) != cudaSuccess) {
error = std::string("cudaMalloc failed in ") + label;
clear\_kzg\_points\_cache(cache);
return false;
}
if (cudaMemcpy(cache.device\_ptr, host\_ptr, count \* sizeof(G1Affine), cudaMemcpyHostToDevice) !=
cudaSuccess) {
error = std::string("cudaMemcpy failed in ") + label;
clear\_kzg\_points\_cache(cache);
return false;
}
cache.host\_ptr = host\_ptr;
cache.capacity = count;
return true;
}
inline bool ensure\_kzg\_work\_cache(size\_t count, const char \*label, std::string &error) {
if (kzg\_work\_cache.capacity >= count) {
return true;
}
clear\_kzg\_work\_cache();
if (cudaMalloc(&kzg\_work\_cache.d\_scalars, count \* sizeof(Fr)) != cudaSuccess ||
cudaMalloc(&kzg\_work\_cache.d\_partials, count \* sizeof(G1Jacobian)) != cudaSuccess ||
cudaMalloc(&kzg\_work\_cache.d\_reduce, count \* sizeof(G1Jacobian)) != cudaSuccess) {
error = std::string("cudaMalloc failed in ") + label;
clear\_kzg\_work\_cache();
return false;
}
kzg\_work\_cache.capacity = count;
return true;
}
inline bool kzg\_commit\_impl(const G1Affine \*points, const std::vector &scalars, size\_t count,
KzgPointsCache &points\_cache, const char \*label, G1Affine &out,
std::string &error) {
if (count == 0) {
G1Jacobian acc;
g1\_set\_infinity(acc);
g1\_to\_affine(out, acc);
return true;
}
cudaStream\_t stream = nullptr;
if (!ensure\_kzg\_points\_cache(points\_cache, points, count, label, error)) {
return false;
}
if (!ensure\_kzg\_work\_cache(count, label, error)) {
return false;
}
if (cudaMemcpyAsync(kzg\_work\_cache.d\_scalars, scalars.data(), count \* sizeof(Fr),
cudaMemcpyHostToDevice, stream) != cudaSuccess) {
error = std::string("cudaMemcpyAsync failed in ") + label;
return false;
}
cudaError\_t launch = msm\_partial\_launch(points\_cache.device\_ptr, kzg\_work\_cache.d\_scalars,
static\_cast(count), kzg\_work\_cache.d\_partials,
stream);
if (launch != cudaSuccess) {
error = std::string("msm\_partial\_launch failed in ") + label;
return false;
}
int current = static\_cast(count);
G1Jacobian \*d\_in = kzg\_work\_cache.d\_partials;
G1Jacobian \*d\_out = kzg\_work\_cache.d\_reduce;
while (current > 1) {
cudaError\_t reduce\_launch = msm\_reduce\_launch(d\_in, d\_out, current, stream);
if (reduce\_launch != cudaSuccess) {
error = std::string("msm\_reduce\_launch failed in ") + label;
return false;
}
current = (current + 1) / 2;
std::swap(d\_in, d\_out);
}
G1Jacobian jac;
if (cudaMemcpyAsync(&jac, d\_in, sizeof(G1Jacobian), cudaMemcpyDeviceToHost, stream) !=
cudaSuccess) {
error = std::string("cudaMemcpyAsync failed in ") + label;
return false;
}
cudaStreamSynchronize(stream);
G1Affine aff;
g1\_to\_affine(aff, jac);
out = aff;
return true;
}
inline bool kzg\_commit(const ParamsKZG ¶ms, const std::vector &coeffs, G1Affine &out,
std::string &error) {
if (coeffs.size() > params.g1\_monomial.size()) {
error = "polynomial degree exceeds SRS size";
return false;
}
size\_t count = coeffs.size();
return kzg\_commit\_impl(params.g1\_monomial.data(), coeffs, count, kzg\_monomial\_cache,
"kzg\_commit", out, error);
}
inline bool kzg\_commit\_lagrange(const ParamsKZG ¶ms, const std::vector &values,
G1Affine &out, std::string &error) {
if (values.size() > params.g1\_lagrange.size()) {
error = "lagrange polynomial degree exceeds SRS size";
return false;
}
size\_t count = values.size();
return kzg\_commit\_impl(params.g1\_lagrange.data(), values, count, kzg\_lagrange\_cache,
"kzg\_commit\_lagrange", out, error);
}
inline bool kzg\_open(const ParamsKZG ¶ms, const std::vector &coeffs, const Fr &z,
Fr &eval, G1Affine &proof, std::string &error) {
eval = poly\_eval(coeffs, z);
std::vector shifted = coeffs;
if (!shifted.empty()) {
fr\_sub(shifted[0], shifted[0], eval);
}
std::vector quotient = poly\_divide\_by\_linear(shifted, z);
if (!kzg\_commit(params, quotient, proof, error)) {
return false;
}
return true;
}
struct ProverQuery {
Fr point;
const PolynomialFr \*poly;
};
struct CommitmentData {
const PolynomialFr \*poly = nullptr;
size\_t set\_index = 0;
std::vector point\_indices;
std::vector evals;
};
inline std::array fr\_to\_key(const Fr &value) {
std::array out{};
uint8\_t tmp[32];
fr\_to\_bytes(tmp, value);
for (size\_t i = 0; i < 32; ++i) {
out[i] = tmp[31 - i];
}
return out;
}
inline bool kzg\_debug\_enabled() {
const char \*env = std::getenv("MIDNIGHT\_CUDA\_KZG\_DEBUG");
return env && std::strcmp(env, "1") == 0;
}
inline void fr\_to\_hex\_be(const Fr &value, char out[65]) {
uint8\_t tmp[32];
fr\_to\_bytes(tmp, value);
for (size\_t i = 0; i < 32; ++i) {
uint8\_t byte = tmp[31 - i];
std::snprintf(out + i \* 2, 3, "%02x", byte);
}
out[64] = '\0';
}
inline void debug\_log\_fr(const char \*label, const Fr &value) {
if (!kzg\_debug\_enabled()) {
return;
}
char hex[65];
fr\_to\_hex\_be(value, hex);
std::fprintf(stderr, "kzg debug: %s=%s\n", label, hex);
}
inline void debug\_log\_point\_sets(const std::vector> &point\_sets,
const char \*label) {
if (!kzg\_debug\_enabled()) {
return;
}
constexpr size\_t max\_points = 8;
std::fprintf(stderr, "kzg debug: %s sets=%zu\n", label, point\_sets.size());
for (size\_t idx = 0; idx < point\_sets.size(); ++idx) {
const auto &set = point\_sets[idx];
std::fprintf(stderr, "kzg debug: %s set %zu len=%zu points=", label, idx, set.size());
for (size\_t i = 0; i < set.size() && i < max\_points; ++i) {
char hex[65];
fr\_to\_hex\_be(set[i], hex);
std::fprintf(stderr, "%s%s", (i > 0) ? "," : "", hex);
}
if (set.size() > max\_points) {
std::fprintf(stderr, "...(+%zu)", set.size() - max\_points);
}
std::fprintf(stderr, "\n");
}
}
inline PolynomialFr poly\_inner\_product(const std::vector &polys, const Fr &x) {
PolynomialFr acc;
if (polys.empty()) {
return acc;
}
size\_t max\_len = 0;
for (const auto &poly : polys) {
if (poly.values.size() > max\_len) {
max\_len = poly.values.size();
}
}
if (max\_len == 0) {
return acc;
}
acc.values.assign(max\_len, Fr{});
std::vector powers(polys.size());
if (!powers.empty()) {
powers[0] = fr\_one();
for (size\_t i = 1; i < powers.size(); ++i) {
fr\_mul(powers[i], powers[i - 1], x);
}
}
constexpr size\_t kMinChunk = 2048;
parallel\_for\_chunks(max\_len, kMinChunk, [&](size\_t start, size\_t end) {
for (size\_t idx = start; idx < end; ++idx) {
Fr sum{};
for (size\_t i = 0; i < polys.size(); ++i) {
if (idx >= polys[i].values.size()) {
continue;
}
Fr term;
fr\_mul(term, polys[i].values[idx], powers[i]);
fr\_add(sum, sum, term);
}
acc.values[idx] = sum;
}
});
return acc;
}
inline std::vector evals\_inner\_product(const std::vector> &evals,
const Fr &x) {
std::vector acc;
if (evals.empty()) {
return acc;
}
size\_t max\_len = 0;
for (const auto &eval\_set : evals) {
if (eval\_set.size() > max\_len) {
max\_len = eval\_set.size();
}
}
if (max\_len == 0) {
return acc;
}
acc.assign(max\_len, Fr{});
std::vector powers(evals.size());
if (!powers.empty()) {
powers[0] = fr\_one();
for (size\_t i = 1; i < powers.size(); ++i) {
fr\_mul(powers[i], powers[i - 1], x);
}
}
constexpr size\_t kMinChunk = 2048;
parallel\_for\_chunks(max\_len, kMinChunk, [&](size\_t start, size\_t end) {
for (size\_t idx = start; idx < end; ++idx) {
Fr sum{};
for (size\_t i = 0; i < evals.size(); ++i) {
if (idx >= evals[i].size()) {
continue;
}
Fr term;
fr\_mul(term, evals[i][idx], powers[i]);
fr\_add(sum, sum, term);
}
acc[idx] = sum;
}
});
return acc;
}
inline bool construct\_intermediate\_sets(
const std::vector &queries, std::vector &commitment\_map,
std::vector> &point\_sets, std::string &error) {
std::map, size\_t> point\_index\_map;
std::vector point\_values;
for (const auto &query : queries) {
auto key = fr\_to\_key(query.point);
auto it = point\_index\_map.find(key);
size\_t point\_idx = 0;
if (it == point\_index\_map.end()) {
point\_idx = point\_index\_map.size();
point\_index\_map[key] = point\_idx;
point\_values.push\_back(query.point);
} else {
point\_idx = it->second;
}
auto existing = std::find\_if(commitment\_map.begin(), commitment\_map.end(),
[&](const CommitmentData &data) { return data.poly == query.poly; });
if (existing != commitment\_map.end()) {
if (std::find(existing->point\_indices.begin(), existing->point\_indices.end(), point\_idx) !=
existing->point\_indices.end()) {
error = "duplicate query";
return false;
}
existing->point\_indices.push\_back(point\_idx);
} else {
CommitmentData data;
data.poly = query.poly;
data.point\_indices.push\_back(point\_idx);
commitment\_map.push\_back(std::move(data));
}
}
std::map, size\_t> point\_idx\_sets;
std::vector>> commitment\_set\_map;
for (const auto &data : commitment\_map) {
std::set idx\_set(data.point\_indices.begin(), data.point\_indices.end());
commitment\_set\_map.push\_back({data.poly, idx\_set});
if (point\_idx\_sets.find(idx\_set) == point\_idx\_sets.end()) {
size\_t next\_index = point\_idx\_sets.size();
point\_idx\_sets.emplace(idx\_set, next\_index);
}
}
for (auto &data : commitment\_map) {
data.evals.assign(data.point\_indices.size(), Fr{});
}
for (const auto &query : queries) {
auto key = fr\_to\_key(query.point);
auto point\_idx\_it = point\_index\_map.find(key);
if (point\_idx\_it == point\_index\_map.end()) {
error = "missing point index";
return false;
}
size\_t point\_idx = point\_idx\_it->second;
std::set idx\_set;
for (const auto &entry : commitment\_set\_map) {
if (entry.first == query.poly) {
idx\_set = entry.second;
break;
}
}
if (idx\_set.empty()) {
error = "missing commitment set";
return false;
}
auto set\_it = point\_idx\_sets.find(idx\_set);
if (set\_it == point\_idx\_sets.end()) {
error = "missing point set index";
return false;
}
size\_t set\_index = set\_it->second;
std::vector idx\_vec(idx\_set.begin(), idx\_set.end());
auto pos\_it = std::find(idx\_vec.begin(), idx\_vec.end(), point\_idx);
if (pos\_it == idx\_vec.end()) {
error = "point index missing in set";
return false;
}
size\_t point\_pos = static\_cast(pos\_it - idx\_vec.begin());
for (auto &data : commitment\_map) {
if (data.poly == query.poly) {
data.set\_index = set\_index;
Fr eval = poly\_eval(query.poly->values, query.point);
if (point\_pos >= data.evals.size()) {
error = "eval index out of range";
return false;
}
data.evals[point\_pos] = eval;
}
}
}
point\_sets.assign(point\_idx\_sets.size(), {});
for (const auto &entry : point\_idx\_sets) {
size\_t set\_index = entry.second;
for (auto idx : entry.first) {
if (idx >= point\_values.size()) {
error = "point index out of range";
return false;
}
point\_sets[set\_index].push\_back(point\_values[idx]);
}
}
return true;
}
inline bool kzg\_multi\_open(const ParamsKZG ¶ms, const std::vector &queries,
ProofTranscript &transcript, std::string &error) {
if (queries.empty()) {
error = "kzg\_multi\_open requires queries";
return false;
}
Fr x1 = transcript.challenge\_fr();
Fr x2 = transcript.challenge\_fr();
if (kzg\_debug\_enabled()) {
std::fprintf(stderr, "kzg debug: multi\_open queries=%zu\n", queries.size());
debug\_log\_fr("multi\_open x1", x1);
debug\_log\_fr("multi\_open x2", x2);
}
std::vector commitment\_map;
std::vector> point\_sets;
if (!construct\_intermediate\_sets(queries, commitment\_map, point\_sets, error)) {
return false;
}
if (kzg\_debug\_enabled()) {
debug\_log\_point\_sets(point\_sets, "multi\_open point\_sets");
std::fprintf(stderr, "kzg debug: multi\_open commitment\_map len=%zu\n",
commitment\_map.size());
for (size\_t idx = 0; idx < commitment\_map.size(); ++idx) {
const auto &data = commitment\_map[idx];
std::fprintf(stderr, "kzg debug: multi\_open commitment %zu set\_index=%zu points=[", idx,
data.set\_index);
for (size\_t j = 0; j < data.point\_indices.size(); ++j) {
std::fprintf(stderr, "%s%zu", (j > 0) ? "," : "", data.point\_indices[j]);
}
std::fprintf(stderr, "]\n");
}
}
std::vector> q\_polys(point\_sets.size());
for (const auto &data : commitment\_map) {
if (data.poly == nullptr) {
error = "invalid polynomial in commitment";
return false;
}
if (data.set\_index >= q\_polys.size()) {
error = "commitment set index out of range";
return false;
}
q\_polys[data.set\_index].push\_back(\*data.poly);
}
std::vector q\_poly\_sets;
q\_poly\_sets.reserve(q\_polys.size());
for (const auto &polys : q\_polys) {
q\_poly\_sets.push\_back(poly\_inner\_product(polys, x1));
}
std::vector f\_polys;
f\_polys.resize(point\_sets.size());
size\_t target\_len = static\_cast(1ULL << params.k);
if (!point\_sets.empty()) {
constexpr size\_t kMinChunk = 1;
parallel\_for\_chunks(point\_sets.size(), kMinChunk, [&](size\_t start, size\_t end) {
std::vector scratch;
for (size\_t i = start; i < end; ++i) {
PolynomialFr poly = q\_poly\_sets[i];
scratch.clear();
scratch.reserve(poly.values.size());
for (const auto &point : point\_sets[i]) {
poly\_divide\_by\_linear\_into(poly.values, point, scratch);
poly.values.swap(scratch);
}
poly.values.resize(target\_len);
f\_polys[i] = std::move(poly);
}
});
}
PolynomialFr f\_poly = poly\_inner\_product(f\_polys, x2);
G1Affine f\_commitment;
if (!kzg\_commit(params, f\_poly.values, f\_commitment, error)) {
return false;
}
transcript.write\_g1(f\_commitment);
Fr x3 = transcript.challenge\_fr();
if (kzg\_debug\_enabled()) {
debug\_log\_fr("multi\_open x3", x3);
}
std::vector q\_evals(q\_poly\_sets.size());
if (!q\_poly\_sets.empty()) {
constexpr size\_t kMinChunk = 1;
parallel\_for\_chunks(q\_poly\_sets.size(), kMinChunk, [&](size\_t start, size\_t end) {
for (size\_t i = start; i < end; ++i) {
q\_evals[i] = poly\_eval(q\_poly\_sets[i].values, x3);
}
});
}
for (const auto &eval : q\_evals) {
if (kzg\_debug\_enabled()) {
debug\_log\_fr("multi\_open q\_eval\_on\_x3", eval);
}
transcript.write\_fr(eval);
}
Fr x4 = transcript.challenge\_fr();
if (kzg\_debug\_enabled()) {
debug\_log\_fr("multi\_open x4", x4);
}
std::vector final\_polys = q\_poly\_sets;
final\_polys.push\_back(f\_poly);
PolynomialFr final\_poly = poly\_inner\_product(final\_polys, x4);
Fr v = poly\_eval(final\_poly.values, x3);
std::vector shifted = final\_poly.values;
if (!shifted.empty()) {
fr\_sub(shifted[0], shifted[0], v);
}
std::vector pi\_coeffs = poly\_divide\_by\_linear(shifted, x3);
G1Affine pi\_commitment;
if (!kzg\_commit(params, pi\_coeffs, pi\_commitment, error)) {
return false;
}
transcript.write\_g1(pi\_commitment);
return true;
}
} // namespace midnight\_plonk