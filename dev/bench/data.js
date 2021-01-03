window.BENCHMARK_DATA = {
  "lastUpdate": 1609691667702,
  "repoUrl": "https://github.com/vorner/arc-swap",
  "entries": {
    "Track benchmarks": [
      {
        "commit": {
          "author": {
            "name": "vorner",
            "username": "vorner"
          },
          "committer": {
            "name": "vorner",
            "username": "vorner"
          },
          "id": "18cacb53939503210e7598993eef6b87fc8834b2",
          "message": "Keep benchmarks in GH pages",
          "timestamp": "2021-01-02T19:13:52Z",
          "url": "https://github.com/vorner/arc-swap/pull/51/commits/18cacb53939503210e7598993eef6b87fc8834b2"
        },
        "date": 1609689524602,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 17,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 33,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 69,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 121,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/cache",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load",
            "value": 27,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 38,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 99,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 928,
            "range": "± 413",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load",
            "value": 78,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 140,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 178,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 915,
            "range": "± 94",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "vorner@vorner.cz",
            "name": "Michal 'vorner' Vaner",
            "username": "vorner"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "10355d69139fa26193de364012d1e5ca8614012d",
          "message": "Merge pull request #51 from vorner/keep-benchmarks\n\nKeep benchmarks in GH pages",
          "timestamp": "2021-01-03T17:27:54+01:00",
          "tree_id": "bf078b66a15959df8a86ab88f8a6b53d81bafa52",
          "url": "https://github.com/vorner/arc-swap/commit/10355d69139fa26193de364012d1e5ca8614012d"
        },
        "date": 1609691667321,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 18,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 29,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 59,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 105,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/cache",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load",
            "value": 28,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 46,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 101,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 900,
            "range": "± 284",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/cache",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load",
            "value": 102,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 150,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 209,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1085,
            "range": "± 111",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}