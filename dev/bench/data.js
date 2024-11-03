window.BENCHMARK_DATA = {
  "lastUpdate": 1730644736733,
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
      },
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
          "id": "ae37017e3319215f5ba99e8e46ce414ba6c228e4",
          "message": "Allow mapping of dyn-access",
          "timestamp": "2021-04-29T06:43:37Z",
          "url": "https://github.com/vorner/arc-swap/pull/53/commits/ae37017e3319215f5ba99e8e46ce414ba6c228e4"
        },
        "date": 1621448861871,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 20,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 36,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 51,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 141,
            "range": "± 11",
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
            "value": 35,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 51,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 100,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 836,
            "range": "± 329",
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
            "value": 80,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 127,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 157,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1034,
            "range": "± 75",
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
            "name": "vorner",
            "username": "vorner"
          },
          "committer": {
            "name": "vorner",
            "username": "vorner"
          },
          "id": "fc733c404ba45ba60cd61734db313629a659c67e",
          "message": "Solve some lints",
          "timestamp": "2021-04-29T06:43:37Z",
          "url": "https://github.com/vorner/arc-swap/pull/54/commits/fc733c404ba45ba60cd61734db313629a659c67e"
        },
        "date": 1621449912177,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 15,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 26,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 40,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 126,
            "range": "± 0",
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
            "value": 19,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 40,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 51,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1185,
            "range": "± 415",
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
            "value": 86,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 138,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 181,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1092,
            "range": "± 20",
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
          "id": "6a58a01790ade363dd88dd84f93ca12b1563f739",
          "message": "Merge pull request #54 from vorner/warnings\n\nSolve some lints",
          "timestamp": "2021-05-19T20:54:39+02:00",
          "tree_id": "c664bd5abee00da29bf69e17485d1a9ff4fca44c",
          "url": "https://github.com/vorner/arc-swap/commit/6a58a01790ade363dd88dd84f93ca12b1563f739"
        },
        "date": 1621450960021,
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
            "value": 37,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 53,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 156,
            "range": "± 5",
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
            "value": 22,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 48,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 73,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 886,
            "range": "± 374",
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
            "value": 102,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 112,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 166,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1020,
            "range": "± 90",
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
            "name": "vorner",
            "username": "vorner"
          },
          "committer": {
            "name": "vorner",
            "username": "vorner"
          },
          "id": "f13cec6fcf1187080c6d7b7eeae5770070ff8bf3",
          "message": "Allow mapping of dyn-access",
          "timestamp": "2021-05-19T18:54:42Z",
          "url": "https://github.com/vorner/arc-swap/pull/53/commits/f13cec6fcf1187080c6d7b7eeae5770070ff8bf3"
        },
        "date": 1621451050676,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 20,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 35,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 154,
            "range": "± 11",
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
            "value": 23,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 52,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 85,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1300,
            "range": "± 638",
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
            "value": 85,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 130,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 166,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1085,
            "range": "± 74",
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
          "id": "376beed33d27a63416e1f468a538a71ec1add16f",
          "message": "Merge pull request #53 from vorner/dyn-access\n\nAllow mapping of dyn-access",
          "timestamp": "2021-05-19T21:35:05+02:00",
          "tree_id": "afa685fdf5be83630d46797af0a22c22416f090c",
          "url": "https://github.com/vorner/arc-swap/commit/376beed33d27a63416e1f468a538a71ec1add16f"
        },
        "date": 1621453286563,
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 54,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 157,
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
            "value": 22,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 46,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 83,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1060,
            "range": "± 457",
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
            "value": 104,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 136,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 164,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1069,
            "range": "± 69",
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
            "name": "vorner",
            "username": "vorner"
          },
          "committer": {
            "name": "vorner",
            "username": "vorner"
          },
          "id": "12ed8ac1bd5cb6cc6adf5b97754ac94ef919fa5b",
          "message": "Don't leave threads running in tests",
          "timestamp": "2021-05-20T22:25:32Z",
          "url": "https://github.com/vorner/arc-swap/pull/55/commits/12ed8ac1bd5cb6cc6adf5b97754ac94ef919fa5b"
        },
        "date": 1621794271873,
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
            "value": 34,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 50,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 143,
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
            "value": 31,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 58,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 71,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1341,
            "range": "± 476",
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
            "value": 93,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 148,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 164,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1074,
            "range": "± 93",
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
          "id": "c914b2866922a18d020281d4a84cff934db1bf49",
          "message": "Merge pull request #55 from vorner/dont-leave-threads\n\nDon't leave threads running in tests",
          "timestamp": "2021-05-23T20:40:43+02:00",
          "tree_id": "98bbcef1f0829b63b315c1d12799bc1cd15b40ac",
          "url": "https://github.com/vorner/arc-swap/commit/c914b2866922a18d020281d4a84cff934db1bf49"
        },
        "date": 1621795609978,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 23,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 40,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 60,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 176,
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
            "value": 36,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 60,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 87,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1049,
            "range": "± 415",
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
            "value": 91,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 144,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 178,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1143,
            "range": "± 243",
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
          "id": "e9ab72693098bedb37603b3ecfbb01287f89ce02",
          "message": "Changelog",
          "timestamp": "2021-05-23T20:42:23+02:00",
          "tree_id": "adcaddc0011bf31ae95c93cd38a3ea2e56621472",
          "url": "https://github.com/vorner/arc-swap/commit/e9ab72693098bedb37603b3ecfbb01287f89ce02"
        },
        "date": 1621795685990,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 50,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 149,
            "range": "± 9",
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
            "value": 25,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 63,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 73,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1374,
            "range": "± 409",
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
            "value": 84,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 109,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 156,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1133,
            "range": "± 41",
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
            "name": "vorner",
            "username": "vorner"
          },
          "committer": {
            "name": "vorner",
            "username": "vorner"
          },
          "id": "0080968b798f6f8c79a1c64705f9a471c9adcbb1",
          "message": "Cache doc example added",
          "timestamp": "2021-08-17T18:54:34Z",
          "url": "https://github.com/vorner/arc-swap/pull/58/commits/0080968b798f6f8c79a1c64705f9a471c9adcbb1"
        },
        "date": 1629658696519,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 36,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 53,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 160,
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
            "value": 28,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 65,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 75,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1018,
            "range": "± 361",
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
            "value": 85,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 117,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 157,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1103,
            "range": "± 44",
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
            "name": "vorner",
            "username": "vorner"
          },
          "committer": {
            "name": "vorner",
            "username": "vorner"
          },
          "id": "b3d174774b159353a402c9c3940d46d62f5b88b4",
          "message": "Cache doc example added",
          "timestamp": "2021-08-17T18:54:34Z",
          "url": "https://github.com/vorner/arc-swap/pull/58/commits/b3d174774b159353a402c9c3940d46d62f5b88b4"
        },
        "date": 1629659287877,
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
            "value": 33,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 48,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 147,
            "range": "± 11",
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
            "value": 31,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 62,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 93,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 851,
            "range": "± 354",
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
            "value": 70,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 130,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 150,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1100,
            "range": "± 78",
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
          "id": "444eaef04bd4601964ffb1b97d446a3d2e6aec1f",
          "message": "Merge pull request #58 from vorner/cache-doc-thead-local\n\nCache doc example added",
          "timestamp": "2021-08-22T21:27:30+02:00",
          "tree_id": "85760053f66b18344d0b3b2e023a3e213669f943",
          "url": "https://github.com/vorner/arc-swap/commit/444eaef04bd4601964ffb1b97d446a3d2e6aec1f"
        },
        "date": 1629660953918,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 21,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 38,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 55,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 169,
            "range": "± 11",
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
            "value": 23,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 43,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 87,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1406,
            "range": "± 445",
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
            "value": 76,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 121,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 161,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1189,
            "range": "± 116",
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
          "id": "1484d8ca80b5bf55493d648ea04165f842e4e8f5",
          "message": "Version bump to 1.3.1",
          "timestamp": "2021-08-22T21:29:25+02:00",
          "tree_id": "e773486bbf4b8e5a11d11949894d81db6d95081d",
          "url": "https://github.com/vorner/arc-swap/commit/1484d8ca80b5bf55493d648ea04165f842e4e8f5"
        },
        "date": 1629661088228,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 22,
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
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 172,
            "range": "± 0",
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
            "value": 22,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 48,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 100,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1126,
            "range": "± 623",
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
            "value": 134,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 194,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 208,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1331,
            "range": "± 624",
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
          "id": "d57e1ea6f7bb1ebe71535d5011db82197864bf95",
          "message": "Document that AsRef isn't for owned things\n\nConflicting trait implementations :-(\n\nCloses #61.",
          "timestamp": "2021-08-27T17:04:41+02:00",
          "tree_id": "a9e4daf616698e3e41f9b798ff7a20c15f19cecf",
          "url": "https://github.com/vorner/arc-swap/commit/d57e1ea6f7bb1ebe71535d5011db82197864bf95"
        },
        "date": 1630077202837,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 22,
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
            "value": 50,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 147,
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
            "value": 25,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 42,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 70,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1250,
            "range": "± 447",
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
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 163,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 177,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1320,
            "range": "± 40",
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
          "id": "a87221bc2edb9d0aea1cddd30402330733b5893c",
          "message": "GH: Update codecov action",
          "timestamp": "2021-09-12T09:26:42+02:00",
          "tree_id": "544e5702a94b739714f025b6122ae507919ccb58",
          "url": "https://github.com/vorner/arc-swap/commit/a87221bc2edb9d0aea1cddd30402330733b5893c"
        },
        "date": 1631432015932,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 20,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 33,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 44,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 126,
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
            "value": 27,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 54,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 74,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1010,
            "range": "± 365",
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
            "value": 87,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 130,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 143,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 973,
            "range": "± 70",
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
            "name": "vorner",
            "username": "vorner"
          },
          "committer": {
            "name": "vorner",
            "username": "vorner"
          },
          "id": "3043a39d1de95f9355dedde50860f7f3d4c2ef3e",
          "message": "Const ArcSwapOption initialization",
          "timestamp": "2021-09-12T07:26:54Z",
          "url": "https://github.com/vorner/arc-swap/pull/63/commits/3043a39d1de95f9355dedde50860f7f3d4c2ef3e"
        },
        "date": 1631807629188,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 22,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 35,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 48,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 145,
            "range": "± 5",
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
            "value": 49,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 72,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1050,
            "range": "± 491",
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
            "value": 79,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 116,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 150,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1031,
            "range": "± 65",
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
          "id": "63bda3a59b855d88ad133f81889964c65b5acbc3",
          "message": "Merge pull request #63 from vorner/const-arc-swap-option\n\nConst ArcSwapOption initialization",
          "timestamp": "2021-09-17T11:57:28+02:00",
          "tree_id": "e456fd03a4517ae6319cd5b5895dc5d1890c379e",
          "url": "https://github.com/vorner/arc-swap/commit/63bda3a59b855d88ad133f81889964c65b5acbc3"
        },
        "date": 1631873019352,
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
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 46,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 147,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load",
            "value": 32,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 37,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 65,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 997,
            "range": "± 719",
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
            "value": 107,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 135,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 165,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1196,
            "range": "± 31",
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
          "id": "a8a3ddf07ef36ef6ad086f1c095624130afcf052",
          "message": "Merge pull request #64 from Cyborus04/master\n\nFix typo in `ArcSwapOption::const_empty` documentation",
          "timestamp": "2021-09-19T12:17:53+02:00",
          "tree_id": "70688ae8d6bf7801695767a4efe555b485b10c26",
          "url": "https://github.com/vorner/arc-swap/commit/a8a3ddf07ef36ef6ad086f1c095624130afcf052"
        },
        "date": 1632046961264,
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
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 46,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 144,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load",
            "value": 32,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 30,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 68,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1382,
            "range": "± 1065",
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
            "value": 106,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 141,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 159,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1238,
            "range": "± 19",
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
          "id": "13e5e3f002dc51b3d938cd1e30db139e3fc96278",
          "message": "Tests on pull requests too",
          "timestamp": "2021-11-10T20:12:04+01:00",
          "tree_id": "51b139c3b919a0101ec0c1638941d903ae886c5d",
          "url": "https://github.com/vorner/arc-swap/commit/13e5e3f002dc51b3d938cd1e30db139e3fc96278"
        },
        "date": 1636572024054,
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
            "value": 34,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 55,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 156,
            "range": "± 11",
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
            "value": 26,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 39,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 76,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1141,
            "range": "± 404",
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
            "value": 77,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 101,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 145,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1071,
            "range": "± 71",
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
            "email": "bratsinot@gmail.com",
            "name": "Aleksander",
            "username": "BratSinot"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8d6b1fb599bf4b1fb9114291d6cace2beacbc46e",
          "message": "Add serialize / deserialize features. (#65)\n\nAdd serde support",
          "timestamp": "2021-11-13T20:44:33+01:00",
          "tree_id": "7daa2b959cd284c2657d27b6e950c16824e64bbe",
          "url": "https://github.com/vorner/arc-swap/commit/8d6b1fb599bf4b1fb9114291d6cace2beacbc46e"
        },
        "date": 1636833036179,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 21,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 39,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 49,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 145,
            "range": "± 11",
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
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 58,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 68,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1101,
            "range": "± 486",
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
            "value": 89,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 126,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 158,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1113,
            "range": "± 70",
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
            "name": "vorner",
            "username": "vorner"
          },
          "committer": {
            "name": "vorner",
            "username": "vorner"
          },
          "id": "e411d788f9d5d4cfc20bb109f963312c1124b2c2",
          "message": "Release 1.5.0",
          "timestamp": "2021-11-14T02:35:26Z",
          "url": "https://github.com/vorner/arc-swap/pull/66/commits/e411d788f9d5d4cfc20bb109f963312c1124b2c2"
        },
        "date": 1636878876460,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 22,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 40,
            "range": "± 1",
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
            "value": 153,
            "range": "± 6",
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
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 62,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 82,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1238,
            "range": "± 546",
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
            "value": 83,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 123,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 162,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1183,
            "range": "± 109",
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
          "id": "7e16ca3f778e29b14f093a036f3ba39ee89b6b41",
          "message": "Merge pull request #66 from vorner/next\n\nRelease 1.5.0",
          "timestamp": "2021-11-15T18:41:56+01:00",
          "tree_id": "eaea8185b660ba3b26f2197ffe1b3ed33ceeb650",
          "url": "https://github.com/vorner/arc-swap/commit/7e16ca3f778e29b14f093a036f3ba39ee89b6b41"
        },
        "date": 1636998466516,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 20,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 36,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 53,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 149,
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
            "value": 31,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 62,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 58,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1290,
            "range": "± 582",
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
            "value": 83,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 111,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 148,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1107,
            "range": "± 80",
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
          "id": "f06b05123406a0acef420677f031a6df25af3d9f",
          "message": "Disable benchmarks on pull requests\n\nThere's some problem with permissions, needs to be investigated.. but\ndisabling until then.",
          "timestamp": "2021-12-19T16:26:43+01:00",
          "tree_id": "77677b5f9c9a4e70a8d639530aedae944aa7f89e",
          "url": "https://github.com/vorner/arc-swap/commit/f06b05123406a0acef420677f031a6df25af3d9f"
        },
        "date": 1639928130807,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 24,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 42,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 60,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 163,
            "range": "± 14",
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
            "value": 43,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 53,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 69,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1051,
            "range": "± 410",
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
            "value": 94,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 121,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 164,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1273,
            "range": "± 175",
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
            "email": "Jiahao_XU@outlook.com",
            "name": "Jiahao XU",
            "username": "NobodyXu"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d82b9e14a51a047e4283447713ab83541a0eeaa4",
          "message": "Add thread sanitizer to test (#72)",
          "timestamp": "2021-12-20T11:36:06+01:00",
          "tree_id": "787e2a01c313e3e7176d57127242467901a3cc01",
          "url": "https://github.com/vorner/arc-swap/commit/d82b9e14a51a047e4283447713ab83541a0eeaa4"
        },
        "date": 1639996920135,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 20,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 44,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 141,
            "range": "± 5",
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
            "value": 29,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 48,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 54,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1057,
            "range": "± 363",
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
            "value": 77,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 115,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 146,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1067,
            "range": "± 44",
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
          "id": "cc90e97bf2284d183200a58b3b7658af43938fee",
          "message": "Merge branch 'master' of github.com:vorner/arc-swap",
          "timestamp": "2022-01-08T13:19:55+01:00",
          "tree_id": "c864274a4f9dc76c913d0cc60d48941fcd0dc284",
          "url": "https://github.com/vorner/arc-swap/commit/cc90e97bf2284d183200a58b3b7658af43938fee"
        },
        "date": 1641644846976,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 21,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 38,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 48,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 148,
            "range": "± 9",
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
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 55,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 68,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 895,
            "range": "± 390",
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
            "value": 94,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 121,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 164,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1202,
            "range": "± 84",
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
          "id": "f7f192d1161d6451a277bad24494743f77d12173",
          "message": "Updates of devel deps",
          "timestamp": "2022-06-08T21:02:33+02:00",
          "tree_id": "16d23d7c5d32a2d8da727abd29b582571dc57131",
          "url": "https://github.com/vorner/arc-swap/commit/f7f192d1161d6451a277bad24494743f77d12173"
        },
        "date": 1654715479262,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 29,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 109,
            "range": "± 0",
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
            "value": 29,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 50,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 67,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 990,
            "range": "± 382",
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
            "value": 97,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 148,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 191,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1106,
            "range": "± 75",
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
          "distinct": false,
          "id": "f4bbac353fcd8cf5912f6c3270d02322cfab2283",
          "message": "Releasing 1.5.1",
          "timestamp": "2022-07-30T15:14:33+02:00",
          "tree_id": "03a956d8f055bf6f52ec20d82bde38fa3fbb09d0",
          "url": "https://github.com/vorner/arc-swap/commit/f4bbac353fcd8cf5912f6c3270d02322cfab2283"
        },
        "date": 1659206926397,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 142,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load",
            "value": 28,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 43,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 87,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 922,
            "range": "± 386",
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
            "value": 120,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 170,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 210,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1208,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 2,
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
          "id": "7000e07135c55c4687724b13f323a2048cf1c6f3",
          "message": "Merge pull request #82 from atouchet/bdg\n\nUse SPDX license format and remove obsolete badges",
          "timestamp": "2022-12-06T08:43:35+01:00",
          "tree_id": "ffea23f80dfe717ee624bc9ef0c98509873b52ca",
          "url": "https://github.com/vorner/arc-swap/commit/7000e07135c55c4687724b13f323a2048cf1c6f3"
        },
        "date": 1670313059679,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 16,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 29,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 44,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 147,
            "range": "± 13",
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
            "value": 24,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 42,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 64,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1624,
            "range": "± 586",
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
            "value": 74,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 107,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 146,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 711,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 0,
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
          "id": "035ecbb7a88ec8eb21d9a23a6f60a1923f7643c8",
          "message": "Fix clippy",
          "timestamp": "2022-12-06T08:48:42+01:00",
          "tree_id": "5e1a275db9f21a4bb8b546d54deb8b713387d21e",
          "url": "https://github.com/vorner/arc-swap/commit/035ecbb7a88ec8eb21d9a23a6f60a1923f7643c8"
        },
        "date": 1670313368653,
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
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 144,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load",
            "value": 23,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 45,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 82,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1236,
            "range": "± 487",
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
            "value": 71,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 127,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 166,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1139,
            "range": "± 29",
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
          "distinct": false,
          "id": "102bac92a72f46e86f2bc056c5da08b0070af8c0",
          "message": "Dep updates",
          "timestamp": "2022-12-25T10:08:35+01:00",
          "tree_id": "2491466f6ae43c2f34db6766a3f4fb24c08a5974",
          "url": "https://github.com/vorner/arc-swap/commit/102bac92a72f46e86f2bc056c5da08b0070af8c0"
        },
        "date": 1671959946319,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 52,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 158,
            "range": "± 0",
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
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 40,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 64,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1437,
            "range": "± 533",
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
            "value": 55,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 96,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 131,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1592,
            "range": "± 23",
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
          "id": "a41840be5a0dbc1468c847206c5219e93b255b4d",
          "message": "Merge pull request #85 from vorner/box-dyn\n\nBox dyn",
          "timestamp": "2022-12-25T11:44:40+01:00",
          "tree_id": "ef917d19dde9ea3b7a720d707616b68378da773a",
          "url": "https://github.com/vorner/arc-swap/commit/a41840be5a0dbc1468c847206c5219e93b255b4d"
        },
        "date": 1671965285954,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 158,
            "range": "± 0",
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
            "value": 22,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 41,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 87,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1252,
            "range": "± 712",
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
            "value": 49,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 98,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 129,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1393,
            "range": "± 30",
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
          "id": "ca4b62f4208c2b9cbf559f561fa8be47e69a684d",
          "message": "Merge pull request #86 from vorner/ub-ref\n\nUB fixes",
          "timestamp": "2022-12-25T14:02:40+01:00",
          "tree_id": "8609c32bb2daae9f1668969075a76a3d732345de",
          "url": "https://github.com/vorner/arc-swap/commit/ca4b62f4208c2b9cbf559f561fa8be47e69a684d"
        },
        "date": 1671973593204,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 17,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 147,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load",
            "value": 20,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 36,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 80,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1634,
            "range": "± 516",
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
            "value": 75,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 135,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 158,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 944,
            "range": "± 18",
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
            "email": "bratsinot@gmail.com",
            "name": "Aleksander",
            "username": "BratSinot"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "634dd8040c065fb0c3120ae79dc50289af7b4f9c",
          "message": "Use serde_test for unit test instead of serde_json. (#67)",
          "timestamp": "2022-12-25T14:13:41+01:00",
          "tree_id": "a8b6e8c45359fbb1d84c192a81654dc3993c49dc",
          "url": "https://github.com/vorner/arc-swap/commit/634dd8040c065fb0c3120ae79dc50289af7b4f9c"
        },
        "date": 1671974232556,
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
            "value": 32,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 47,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 153,
            "range": "± 9",
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
            "value": 25,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 37,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 81,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1564,
            "range": "± 663",
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
            "value": 70,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 104,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 153,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 849,
            "range": "± 63",
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
          "id": "81a56de7884fc93843cad46c12735cc06df0d6fc",
          "message": "Merge pull request #87 from vorner/future-proof-rc\n\nFuture-proof the Rc::as_ptr too",
          "timestamp": "2022-12-25T17:27:34+01:00",
          "tree_id": "89c93bcfe41b965024ec2c0f1e64422beb16f7a3",
          "url": "https://github.com/vorner/arc-swap/commit/81a56de7884fc93843cad46c12735cc06df0d6fc"
        },
        "date": 1671985869083,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 158,
            "range": "± 0",
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
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 43,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 51,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1464,
            "range": "± 448",
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
            "value": 51,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 89,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 134,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1515,
            "range": "± 33",
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
          "id": "41e7e7df9492ba6f6e8d2910defc7d6edd327e95",
          "message": "Document the bug in two-layer option",
          "timestamp": "2022-12-26T09:24:30+01:00",
          "tree_id": "e737237a92209479dee4a295a67a7a09b84ae5be",
          "url": "https://github.com/vorner/arc-swap/commit/41e7e7df9492ba6f6e8d2910defc7d6edd327e95"
        },
        "date": 1672043243006,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 17,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 28,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 140,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load",
            "value": 32,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 32,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 78,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1499,
            "range": "± 1350",
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
            "value": 76,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 129,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 169,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 1005,
            "range": "± 42",
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
          "id": "2a62e2bf6916c9cc8fb3832abac95b9e3287b9ec",
          "message": "Version 1.6.0",
          "timestamp": "2022-12-31T19:38:41+01:00",
          "tree_id": "c9fc90efd83253f1deb05fa8ea39634f9b2ca6d0",
          "url": "https://github.com/vorner/arc-swap/commit/2a62e2bf6916c9cc8fb3832abac95b9e3287b9ec"
        },
        "date": 1672512129979,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 21,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 37,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 59,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 200,
            "range": "± 6",
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
            "value": 35,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 57,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 79,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 1187,
            "range": "± 586",
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
            "value": 77,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 115,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 169,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 913,
            "range": "± 85",
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
          "id": "20a5b1244a1ab5ef1278f82ff2c3a09dac3b544c",
          "message": "Merge pull request #95 from oriontvv/add-dependabot\n\nAdd dependabot",
          "timestamp": "2024-02-19T21:02:18+01:00",
          "tree_id": "de8f6deaf11968b1c91a9c2e1ad783cfd20260d0",
          "url": "https://github.com/vorner/arc-swap/commit/20a5b1244a1ab5ef1278f82ff2c3a09dac3b544c"
        },
        "date": 1708373226340,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 54,
            "range": "± 0",
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
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 579,
            "range": "± 7",
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
            "value": 53,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 69,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 631,
            "range": "± 16",
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
          "id": "96583fe0c2e4f3167a517eb7ac94b9cd374434d8",
          "message": "Dep updates",
          "timestamp": "2024-02-20T08:22:22+01:00",
          "tree_id": "2d523d2c785b31e547720ee5a331a8576ba0bed7",
          "url": "https://github.com/vorner/arc-swap/commit/96583fe0c2e4f3167a517eb7ac94b9cd374434d8"
        },
        "date": 1708413979985,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 55,
            "range": "± 0",
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
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 22,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 561,
            "range": "± 12",
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
            "value": 48,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 67,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 86,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 612,
            "range": "± 4",
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
          "id": "a6d4b0e4f929648494a615e609d30d2b6a0520e3",
          "message": "More dep updates",
          "timestamp": "2024-02-20T08:32:04+01:00",
          "tree_id": "57e0b182217ee4d5af959eec8fc66924afe1949b",
          "url": "https://github.com/vorner/arc-swap/commit/a6d4b0e4f929648494a615e609d30d2b6a0520e3"
        },
        "date": 1708414643718,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 53,
            "range": "± 0",
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
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 24,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 511,
            "range": "± 6",
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
            "value": 50,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 72,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 88,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 559,
            "range": "± 8",
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
          "id": "facb8395a89fdf3b63637aa0f04acdd1e4dcb77e",
          "message": "Clippy warnings",
          "timestamp": "2024-02-20T20:41:28+01:00",
          "tree_id": "4907d66454c6d78f09c59ea1993a6591d17159fe",
          "url": "https://github.com/vorner/arc-swap/commit/facb8395a89fdf3b63637aa0f04acdd1e4dcb77e"
        },
        "date": 1708458332265,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 53,
            "range": "± 0",
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
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 531,
            "range": "± 1",
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
            "value": 53,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 60,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 82,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 609,
            "range": "± 11",
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
          "id": "c14a467d1f63b121d6f83ff5884cccacf4cf5943",
          "message": "Merge pull request #102 from vorner/dependabot/github_actions/actions/checkout-4\n\nBump actions/checkout from 2 to 4",
          "timestamp": "2024-02-23T16:42:16+01:00",
          "tree_id": "1d29011651924b7090848c15297c373d18bff6de",
          "url": "https://github.com/vorner/arc-swap/commit/c14a467d1f63b121d6f83ff5884cccacf4cf5943"
        },
        "date": 1708703376816,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 56,
            "range": "± 0",
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
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 504,
            "range": "± 3",
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
            "value": 48,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 70,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 82,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 625,
            "range": "± 20",
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
          "id": "1f0bc083d3ec347aeae0fb9e7a25b7e8bc1098ff",
          "message": "Merge pull request #99 from vorner/dependabot/github_actions/rhysd/github-action-benchmark-1.19.3\n\nBump rhysd/github-action-benchmark from 1.8.1 to 1.19.3",
          "timestamp": "2024-02-23T16:42:50+01:00",
          "tree_id": "6504ebae608fe2294bb3663054ff050d49748e16",
          "url": "https://github.com/vorner/arc-swap/commit/1f0bc083d3ec347aeae0fb9e7a25b7e8bc1098ff"
        },
        "date": 1708703411355,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 53,
            "range": "± 0",
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
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 9,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 567,
            "range": "± 1",
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
            "value": 39,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 59,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 527,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "utilities/access-map",
            "value": 6,
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
          "id": "79c22b60c2afddf208646ede0deed94e513fa86c",
          "message": "Merge pull request #98 from vorner/dependabot/github_actions/actions/upload-artifact-4\n\nBump actions/upload-artifact from 2 to 4",
          "timestamp": "2024-02-23T16:43:24+01:00",
          "tree_id": "1bc738d32e2cb5beba65aa641fa8850461f8d220",
          "url": "https://github.com/vorner/arc-swap/commit/79c22b60c2afddf208646ede0deed94e513fa86c"
        },
        "date": 1708703684757,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 53,
            "range": "± 0",
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
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 492,
            "range": "± 3",
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
            "value": 58,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 77,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 88,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 312,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "utilities/access-map",
            "value": 6,
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
          "id": "191993030dd7e926682d678789c3739fbf2e6f6b",
          "message": "Merge pull request #100 from vorner/dependabot/github_actions/actions/cache-4\n\nBump actions/cache from 2 to 4",
          "timestamp": "2024-02-23T16:43:45+01:00",
          "tree_id": "4cf6f5d0d6a8c2699abbe0069e559fd2883432ab",
          "url": "https://github.com/vorner/arc-swap/commit/191993030dd7e926682d678789c3739fbf2e6f6b"
        },
        "date": 1708704069455,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 53,
            "range": "± 0",
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
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 492,
            "range": "± 2",
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
            "value": 56,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 74,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 86,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 489,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "utilities/access-map",
            "value": 6,
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
          "id": "fa0c5cf13990bb346b00e6f84e7d353b61fe6b02",
          "message": "Merge pull request #96 from vorner/dependabot/github_actions/Swatinem/rust-cache-2\n\nBump Swatinem/rust-cache from 1 to 2",
          "timestamp": "2024-02-23T16:44:04+01:00",
          "tree_id": "391c24e166c5a8e31da42ec3d39d4807ed1a26c8",
          "url": "https://github.com/vorner/arc-swap/commit/fa0c5cf13990bb346b00e6f84e7d353b61fe6b02"
        },
        "date": 1708704091000,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 53,
            "range": "± 0",
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
            "value": 12,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 502,
            "range": "± 3",
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
            "value": 58,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 67,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 90,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 440,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "utilities/access-map",
            "value": 6,
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
          "id": "94f378993ad1a2ac3c73c0dd8fee70121ffd6d46",
          "message": "Dep updates",
          "timestamp": "2024-02-23T16:54:50+01:00",
          "tree_id": "b13aa0a0febabc2a46d3abc71fa3626136914201",
          "url": "https://github.com/vorner/arc-swap/commit/94f378993ad1a2ac3c73c0dd8fee70121ffd6d46"
        },
        "date": 1708704348561,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 53,
            "range": "± 0",
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
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 529,
            "range": "± 1",
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
            "value": 49,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 72,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 88,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 607,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "utilities/access-map",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "alex@adnab.me",
            "name": "Alex Auvolat",
            "username": "Alexis211"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "229c7eeef9f4d5b5c664444a478f4aed6e71f18e",
          "message": "no_std support, more minimal and less intrusive version (#93)\n\n* Support building for no_std using nightly Rust compiler\r\n\r\n- Adds a new feature `experimental-thread-local` that enables building for no_std targets.\r\n  If this feature is not enabled, the crate is identical as before,\r\n  still allowing for compilation using Rust stable >= 1.38.0.\r\n\r\n- The `experimental-thread-local` feature makes use of experimental features `thread_local`\r\n  and `lazy_cell`, thus requiring a nightly Rust compiler.\r\n\r\n- Support for `std::sync::RwLock` is dropped in no_std builds.",
          "timestamp": "2024-03-03T13:48:46+01:00",
          "tree_id": "5d50c3f10873a883b15a1ee6ace0e50bf7bebf60",
          "url": "https://github.com/vorner/arc-swap/commit/229c7eeef9f4d5b5c664444a478f4aed6e71f18e"
        },
        "date": 1709470379568,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 53,
            "range": "± 1",
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
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 595,
            "range": "± 60",
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
            "value": 51,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 66,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 86,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 615,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "utilities/access-map",
            "value": 6,
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
          "id": "0554cd01a81d4b2fdf2b74360fb1db0fc090e8c8",
          "message": "Release 1.7.0",
          "timestamp": "2024-03-03T14:00:45+01:00",
          "tree_id": "23e6cd494876423512048e570c64f3565771d640",
          "url": "https://github.com/vorner/arc-swap/commit/0554cd01a81d4b2fdf2b74360fb1db0fc090e8c8"
        },
        "date": 1709471241340,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 53,
            "range": "± 0",
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
            "value": 5,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 22,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 508,
            "range": "± 3",
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
            "value": 54,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 68,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 84,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 464,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "utilities/access-map",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "alex@adnab.me",
            "name": "Alex Auvolat",
            "username": "Alexis211"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2b11a84e5f9e0235125a4f38a5b4e3f8d61df36b",
          "message": "Fix feature list for docs.rs (#112)\n\nPreviously, docs.rs would try to build with all features, but we now\r\nhave exclusive features (internal-test-strategies vs.\r\nexperimental-thread-local) which breaks the build. This fixes the build\r\nby not documenting with the experimental-thread-local feature.",
          "timestamp": "2024-03-23T11:36:05+01:00",
          "tree_id": "6b81e132fe82bda30c8bc66262b5ef16454bff66",
          "url": "https://github.com/vorner/arc-swap/commit/2b11a84e5f9e0235125a4f38a5b4e3f8d61df36b"
        },
        "date": 1711190415523,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 53,
            "range": "± 0",
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
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 22,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 522,
            "range": "± 8",
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
            "value": 48,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 66,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 85,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 532,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "utilities/access-map",
            "value": 6,
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
          "id": "ec5291f4206561f8ecb0f719e61d791cee8f060f",
          "message": "Release 1.7.1",
          "timestamp": "2024-03-23T11:39:57+01:00",
          "tree_id": "1f60e1dcbc4105e922e57431c0468b9de99c6ceb",
          "url": "https://github.com/vorner/arc-swap/commit/ec5291f4206561f8ecb0f719e61d791cee8f060f"
        },
        "date": 1711190656615,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 53,
            "range": "± 0",
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
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 22,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 490,
            "range": "± 2",
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
            "value": 54,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 71,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 83,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 633,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "utilities/access-map",
            "value": 6,
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
          "id": "84bcacea2902d758cd9a7dbd7e4d47ddabbf391b",
          "message": "Merge pull request #115 from vorner/dependabot/github_actions/actions/checkout-4\n\nBump actions/checkout from 2 to 4",
          "timestamp": "2024-04-10T10:03:39+02:00",
          "tree_id": "66ca8ee0f1e4c0d9f69c8bbbdca08bce0fd433e1",
          "url": "https://github.com/vorner/arc-swap/commit/84bcacea2902d758cd9a7dbd7e4d47ddabbf391b"
        },
        "date": 1712736469595,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 53,
            "range": "± 0",
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
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 24,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 549,
            "range": "± 6",
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
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 52,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 87,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 593,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "utilities/access-map",
            "value": 5,
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
          "id": "f7b8753ef8f88bdb6affce28594a4bf1d296db4c",
          "message": "Merge pull request #114 from vorner/dependabot/github_actions/Swatinem/rust-cache-2\n\nBump Swatinem/rust-cache from 1 to 2",
          "timestamp": "2024-04-10T14:45:12+02:00",
          "tree_id": "bf4d36358399e6c6fc805496088789462e36b9d4",
          "url": "https://github.com/vorner/arc-swap/commit/f7b8753ef8f88bdb6affce28594a4bf1d296db4c"
        },
        "date": 1712753321710,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 53,
            "range": "± 0",
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
            "value": 12,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 24,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 530,
            "range": "± 4",
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
            "value": 50,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 67,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 83,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 534,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "utilities/access-map",
            "value": 5,
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
          "id": "ed7fcf5ea30f7b0b05d6c38cfc6823aeee776397",
          "message": "Merge pull request #122 from vorner/dependabot/cargo/parking_lot-0.12.2\n\nBump parking_lot from 0.12.1 to 0.12.2",
          "timestamp": "2024-05-17T19:33:38+02:00",
          "tree_id": "1722de72237d402632f8c3f1f54a0f00282ad6eb",
          "url": "https://github.com/vorner/arc-swap/commit/ed7fcf5ea30f7b0b05d6c38cfc6823aeee776397"
        },
        "date": 1715967472083,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 53,
            "range": "± 0",
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
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 536,
            "range": "± 1",
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
            "value": 50,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 69,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 86,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 501,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "utilities/access-map",
            "value": 4,
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
          "id": "a37dd7a619427870ba2e688ce972f89752dd254d",
          "message": "Merge pull request #121 from vorner/dependabot/cargo/serde-1.0.200\n\nBump serde from 1.0.197 to 1.0.200",
          "timestamp": "2024-05-17T19:44:04+02:00",
          "tree_id": "81b5cc8aceb56bacd6be93f277017edced978506",
          "url": "https://github.com/vorner/arc-swap/commit/a37dd7a619427870ba2e688ce972f89752dd254d"
        },
        "date": 1715968086873,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 52,
            "range": "± 0",
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
            "value": 8,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 544,
            "range": "± 4",
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
            "value": 50,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 57,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 83,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 581,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "utilities/access-map",
            "value": 4,
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
          "id": "d1ba1d36f68ccb8cae1cfdee8d50ab6f925c1c93",
          "message": "Merge pull request #142 from vorner/dependabot/cargo/serde-1.0.214\n\nBump serde from 1.0.200 to 1.0.214",
          "timestamp": "2024-11-03T15:34:02+01:00",
          "tree_id": "6b4e4832dabe22b97964885863d05badad85ffc2",
          "url": "https://github.com/vorner/arc-swap/commit/d1ba1d36f68ccb8cae1cfdee8d50ab6f925c1c93"
        },
        "date": 1730644703991,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 52,
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
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 579,
            "range": "± 12",
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
            "value": 45,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 71,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 78,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 409,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "utilities/access-map",
            "value": 5,
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
          "id": "5cbe73fc0a132bca727ddc961f982f428953d5b5",
          "message": "Merge pull request #141 from vorner/dependabot/cargo/once_cell-1.20.2\n\nBump once_cell from 1.19.0 to 1.20.2",
          "timestamp": "2024-11-03T15:34:26+01:00",
          "tree_id": "95846218c79fce1bf5870bc9946f98562db7da0a",
          "url": "https://github.com/vorner/arc-swap/commit/5cbe73fc0a132bca727ddc961f982f428953d5b5"
        },
        "date": 1730644736346,
        "tool": "cargo",
        "benches": [
          {
            "name": "uncontended/load",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_full",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/load_many",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uncontended/store",
            "value": 53,
            "range": "± 0",
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
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_full",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/load_many",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_loads/store",
            "value": 504,
            "range": "± 1",
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
            "value": 53,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_full",
            "value": 71,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/load_many",
            "value": 77,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/store",
            "value": 356,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_store/cache",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "utilities/access-map",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}