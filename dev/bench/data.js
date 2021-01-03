window.BENCHMARK_DATA = {
  "lastUpdate": 1609694949070,
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
          "id": "64dcf40ad952d87f6f294f9170f272d95bd91f54",
          "message": "Merge pull request #50 from vorner/no-half-lock\n\nReplace the half-lock with helping strategy",
          "timestamp": "2021-01-03T18:01:06+01:00",
          "tree_id": "259ee9c1d6266df99362622b870419e1093cab23",
          "url": "https://github.com/vorner/arc-swap/commit/64dcf40ad952d87f6f294f9170f272d95bd91f54"
        },
        "date": 1609693754486,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 36,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 54,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 176,
            "range": "± 3",
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
            "value": 34,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 56,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 74,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1415,
            "range": "± 1387",
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
            "value": 117,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 190,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 227,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1388,
            "range": "± 35",
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
            "email": "vorner@vorner.cz",
            "name": "Michal 'vorner' Vaner",
            "username": "vorner"
          },
          "distinct": true,
          "id": "0426a36d46ed9ea8473b8ea0b7e0250c958eb54a",
          "message": "Don't fail on slow benchmarks\n\nThese are often more noise than a problem, because they are on CI and we\nhave a lot of them.",
          "timestamp": "2021-01-03T18:22:03+01:00",
          "tree_id": "18a506380ae07625886cd64b876c89d300b7905f",
          "url": "https://github.com/vorner/arc-swap/commit/0426a36d46ed9ea8473b8ea0b7e0250c958eb54a"
        },
        "date": 1609694948652,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 54,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 176,
            "range": "± 4",
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
            "value": 30,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 57,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 79,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1675,
            "range": "± 512",
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
            "value": 97,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 162,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 219,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1050,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}