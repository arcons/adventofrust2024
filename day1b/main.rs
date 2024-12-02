use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let example = "76309   75213
79731   28444
29583   71339
60992   99148
34680   74530
45691   82519
55358   22047
95523   45384
37661   82208
33464   91461
26897   96393
76556   76554
82316   98880
92079   23082
55539   10033
65931   66060
98880   60464
19348   41458
72003   84074
78401   78856
53187   72003
65201   71211
45971   12776
45911   26854
37527   13462
28891   28444
82801   81076
29496   21971
81662   12472
89956   51107
92833   22498
23135   19348
33661   51107
48499   81114
53651   65515
26075   73072
67647   90255
90805   38056
39949   68042
76487   86922
95115   94618
11937   76761
89547   72003
25780   18607
45017   84423
43306   57853
68164   22550
56564   62505
75018   52627
23253   57654
65165   55756
31517   14874
20876   72883
80624   76452
33117   67195
64410   15109
41934   83416
89782   34566
71805   38212
73244   47902
91360   78672
45112   62842
12869   59690
90665   89809
69340   29496
91603   69938
56320   67034
17932   62842
18431   29380
31849   75256
72699   28903
87247   95908
40628   67697
63635   87577
39517   86619
11212   57615
38117   51660
36016   22309
10931   19763
76357   21412
75652   61857
98810   38436
38749   23524
86048   72883
63513   72883
45687   67490
60836   12821
94034   51660
80411   25554
21091   19763
16564   36401
44503   22575
57654   66781
31146   46927
97857   20234
25682   75256
93045   13462
68754   72883
17237   72883
92066   74137
93749   51512
34830   10322
99272   67697
45274   60464
38929   80865
34134   48329
68249   47187
48916   83133
61528   29496
90233   97345
54872   14127
44233   75524
91023   52472
24635   11482
40466   19763
46905   66085
25902   94618
35889   42397
88185   69459
64759   63070
11316   72003
73715   38056
60712   75256
72685   81152
77252   66060
28506   61431
54298   89763
90658   62842
57205   69364
23674   57548
64953   19763
53241   12916
13222   33059
15152   19763
92661   60464
61816   94618
76099   75071
38554   38436
21189   22498
38784   91603
65754   62842
87476   98880
56430   90853
76366   86864
48121   83675
71894   81336
82170   67697
90347   60012
21071   50167
92246   40121
36480   66608
22854   54282
99368   83869
83605   69528
65136   85197
90644   80851
68309   97711
77685   15072
10032   11404
71312   21971
95462   42893
53698   28903
41458   80851
80635   80851
26127   93602
15692   16750
26810   92990
87800   97117
91599   31201
97670   53183
31216   82887
80047   29496
57838   15694
40849   18977
64159   68248
89709   60902
14381   36340
25729   66727
36202   51660
97364   57654
34691   57479
35917   21577
15191   77547
24872   42035
94140   30529
93963   42786
55950   74745
80851   73127
16667   28903
46647   53539
24917   41629
43622   21688
37442   94618
40641   60464
12670   64945
78129   33880
47403   21412
15283   81303
24283   92601
76777   33434
53791   88560
82919   79661
77544   25196
40160   88979
86172   57853
38241   52145
60464   98880
17675   72883
83513   38056
13022   41458
68472   23196
22498   10066
29522   70183
80236   41513
37585   22498
70287   79234
81783   22498
14685   40811
95584   18124
47126   43277
46799   53781
60372   42786
29724   51541
93734   38212
48291   13462
10590   57306
42879   81296
98898   91603
94223   13462
49303   48982
87417   21971
17709   41952
76716   64094
29881   44586
26599   93340
24757   41458
15666   19763
44542   98880
30400   27123
44519   29496
10588   93022
41850   17815
98878   79099
85952   22498
11119   64255
75872   22498
46488   86922
26202   62842
95599   81076
23529   41458
60375   59633
62608   35235
50724   98880
51119   92701
20960   24242
91487   53719
62298   77547
17790   19763
34307   55594
49446   42812
85537   71894
80619   98880
25391   64094
16005   60464
80510   66044
84955   38436
88821   91603
17492   42786
81026   41458
19683   22498
79887   75256
36893   33059
56380   80851
48530   42786
32923   33059
29965   81076
18532   81363
25026   91603
72561   62842
94980   38056
32432   53183
55534   38436
66162   64255
77645   75256
70443   38056
29960   41458
92757   26451
11444   21971
52688   38436
41268   22621
97231   49903
51183   57052
90927   26985
97265   42786
66813   22007
63206   83227
78745   99837
60347   79617
46179   29436
73348   77547
68926   27421
24241   43292
23989   21971
76893   60464
96995   38211
94457   17361
33924   91603
65919   38056
98919   22498
61147   77547
25172   25516
66855   60700
98423   91668
13462   29007
78075   64094
99192   41551
61341   22221
57853   65000
64255   91984
44260   15605
34883   21971
88936   41375
87848   45956
82017   59273
65191   77547
52966   71894
36879   83506
75723   28903
63490   11235
89580   98372
41331   21971
47488   71894
89934   91603
25785   74938
73107   32099
79658   60464
95693   14737
91422   57654
96745   86600
39149   71623
98394   54803
82163   36159
95052   38436
16720   82983
63140   22498
23823   84914
50296   71894
40070   38212
51107   38056
95702   21412
63439   72003
70522   51660
67070   66147
78368   75455
33938   74618
36370   92122
77547   99234
36047   64094
53669   73226
71919   77547
66864   75256
38056   51660
91341   61913
37606   22498
50946   29496
28444   80851
41082   41458
17087   20616
83635   22498
70177   61491
68184   19348
54012   17507
38198   81076
40811   46612
70047   35669
34846   57853
62305   71894
59272   19763
40659   11104
23800   42786
64175   20190
68120   52879
35213   13462
85144   60464
83815   69928
69466   64255
69940   54069
79153   39514
26397   98880
10870   42786
11814   61605
83212   95356
47789   29496
32399   65379
47939   97345
81076   93623
48371   68260
20554   51660
25316   80851
85993   39286
69729   18711
34131   41603
99316   15855
15760   33756
11727   57654
73101   73191
93009   58546
63218   76005
60263   84618
64150   65222
32042   75256
35235   18159
77209   32300
64094   76684
46101   90505
99642   86922
29258   98880
40802   62842
47215   35310
25170   64094
80039   87125
85895   53213
85277   34676
24143   65226
32337   22314
71599   71200
33477   72883
24883   21971
19419   97222
59427   67697
72661   97478
62996   29496
12525   67697
99041   17700
20625   19348
67886   45087
42662   41458
28976   94451
11277   55388
88403   19763
77265   32689
71034   64255
44618   53790
45540   27148
53781   53412
44920   88690
68235   69804
14758   83239
21668   31070
87671   40811
69745   54682
95603   42197
15999   87006
43079   77547
28105   42498
33119   77660
68650   81076
94618   72883
92897   82856
34834   86922
29210   33710
58769   45753
33360   38212
19762   51660
52993   72883
74090   33902
73668   12703
67667   80986
56341   87195
42838   28903
93432   19763
84325   56270
72643   62842
77464   81076
91846   77547
75389   24109
22248   93422
15496   47644
60779   21971
14645   28903
71653   75429
41937   88087
97389   62842
55995   64094
35488   47300
56394   96902
32108   11813
55582   41458
85261   19763
98501   52280
17412   73155
62946   75256
45369   30393
17834   81076
39228   67134
45644   86198
70980   72003
42334   64094
29852   49234
40025   22174
56532   56346
94361   89533
78649   42786
77448   81297
81128   77547
62842   75256
52291   41710
40473   81493
50907   19763
73714   52966
99652   80851
13997   59464
32984   96842
85656   31414
19177   53183
58598   33333
48325   64618
60732   17729
20343   21971
96365   67697
17674   49780
95012   57853
60953   86922
62943   44091
25163   63493
71429   19763
94437   40811
53160   60279
89277   91603
25998   47890
53281   45307
82655   81076
58683   98880
34567   57853
67319   86276
71752   86922
32419   81076
99187   57654
99426   71894
24165   25888
37843   29553
97105   21412
32460   37041
29663   27331
59713   67414
45188   84541
14348   43314
62746   40811
46870   83863
50841   38056
59445   16050
12925   98880
57452   85546
36324   60464
67373   81076
86514   73624
56074   51107
66208   92937
91677   87311
59472   36022
73550   76544
44359   91603
67706   54480
32462   26551
86672   68321
35231   57853
53759   88043
94902   80851
94326   28606
78387   51107
64407   57853
30177   17341
52328   19348
32836   64094
27500   21971
66034   86985
70645   81076
41249   74832
23305   71814
61958   69681
76187   46028
95216   37603
81846   67869
47377   98660
77226   22498
47927   20033
26606   30796
57767   41458
68443   71807
53742   22475
14866   67697
13517   75256
12941   22498
18949   71894
91218   77547
54173   19763
39748   45326
78360   99324
11720   51107
29782   53692
43041   24930
35760   19763
28118   67697
48978   38056
32968   21971
10841   68694
88712   99460
55899   20431
96213   81076
11205   67697
78331   94591
98064   60719
55020   71894
54837   29496
32560   77953
84271   64094
12334   53273
60354   98880
18875   45344
79143   38056
23233   84636
61563   88447
31425   45195
71874   51660
72444   89972
53574   57736
23459   39741
46584   13200
67106   41458
28010   19505
92985   97345
36682   21971
12501   99944
16169   76904
43802   32400
63772   26347
75759   98880
24020   44873
72479   75256
58596   65558
79734   19763
29532   81076
65742   94618
69210   81076
32186   91603
48745   94618
60568   66618
49589   38056
38212   19348
48399   68926
28282   62842
43098   69228
28937   33723
36026   39105
36024   98087
84966   30771
59984   51660
89201   60464
21971   29496
50899   64094
76523   85510
21988   20844
75667   92452
24010   28903
34214   21576
58811   77652
51628   30799
73612   38056
23096   52865
39135   66625
69128   49400
32494   39597
67143   72170
68372   47908
81952   38056
88192   38056
83029   67697
63532   22498
54899   75256
96592   28503
17379   13462
16417   19843
50554   41929
25532   68199
86596   38056
25412   77359
97345   37353
19051   78038
95135   80851
43190   38436
16851   80851
50939   71931
20348   72883
62130   37721
75609   50259
13811   91603
80903   79684
58594   12415
21090   13462
90571   97345
54287   10341
31596   94618
93992   98880
26590   35235
25317   26365
94102   75256
53183   61183
57107   37169
80457   58396
48710   81076
56696   71894
96203   22833
84426   24326
66196   15680
76056   64094
86506   67761
62627   22498
30931   66060
70026   77819
93319   57170
81820   64094
86702   72003
99446   50653
77640   91603
67087   90541
80068   51107
65835   56004
13105   46230
65748   91284
80758   41458
19345   47474
56661   77547
12043   67697
50136   64851
79931   16117
51660   21003
91875   72883
64857   68150
14513   93047
31955   77467
42546   67697
49089   51660
77824   28916
15115   65178
98454   40431
15654   38056
22270   38826
73592   29496
72221   90578
99258   57441
70502   34724
88720   67697
57057   51660
36225   19763
93724   98880
37728   51660
25140   34250
57373   89903
93720   64821
90834   28903
37440   14002
67370   73712
46370   33621
63554   39553
27180   33899
48396   80851
49431   68934
90431   17857
84310   97017
37110   42786
34732   64255
46171   25574
58999   30048
57776   67697
99514   96621
38870   60533
96826   32872
66302   22498
56005   52386
15218   72883
82329   94052
60643   31279
69475   29496
61286   15538
32797   69919
48924   57853
61994   73550
63771   82704
41809   13024
97274   97164
33059   42786
45700   40001
40917   17001
33486   72003
33666   59194
63745   77859
68420   80851
55719   27549
26294   21412
67613   69595
38497   31870
33455   80851
29237   68898
53016   44791
36157   65971
20983   38056
43937   52966
90852   76144
86318   53334
30612   62842
90434   56570
25697   77841
22662   70772
89638   33059
26116   52871
84518   60464
55725   75500
81669   33853
99680   56409
11056   77547
21585   58485
37854   70553
27402   64832
39624   41458
92759   72003
42786   81984
19929   50841
21412   75256
91518   88235
39607   23909
82239   13462
27192   51858
46537   33593
38436   71894
79708   13462
39914   75256
53695   21830
40348   60464
40694   42202
19663   46959
16165   67697
26764   66060
62435   10581
58398   29571
17348   95270
90054   22498
48870   41458
20742   45699
66060   97867
69792   65746
33979   24259
80596   13462
38208   94618
53092   98687
53583   32555
17488   71894
76692   38436
71114   67532
50119   43332
60702   45943
98910   31092
24697   84922
91990   92651
84668   47907
62646   58437
18722   99286
25746   40968
19142   68959
74638   38654
61297   79634
19217   33484
28041   34064
15985   12822
28848   57853
28628   17208
23312   35979
70752   29496
11513   22498
48405   38216
91941   22498
98013   80851
23286   84866
20244   44309
83249   22059
23960   71894
19258   57572
86488   61827
57252   32489
46820   91603
39964   55055
80837   22985
99090   75256
69839   81076
98396   75256
30774   88548
86470   22167
23371   64094
23128   84004
58818   80851
45372   80851
27966   97345
60009   67697
65289   32158
91781   32885
80156   65660
97254   42786
48298   41159
54722   38021
81261   81076
86922   22498
55476   31657
57009   14306
38000   38056
85253   38436
76055   41458
59410   97345
88583   53781
18304   52775
75050   64094
70029   55636
20423   65761
81440   54815
82295   13462
25089   28903
16484   19348
23071   49803
57112   35235
63721   60158
68579   94832
21977   78498
43592   72883
40060   41458
20413   42337
40283   34311
30206   96070
46075   91603
70006   19763
75130   91603
60612   28903
64928   23183
96097   21971
72883   40811
20769   68024
97961   41458
46704   60464
79660   21412
70165   97160
37654   62842
40556   28903
15332   43586
19763   41974
79356   67697
42155   62829
51863   93968
80541   64094
16392   51660
67697   98880
28903   51664
98379   11061
73277   59160
91515   49151
88086   17578
36014   38056
68997   72883
48767   71894
61448   20757
70899   38212
65598   42786
61699   31701
73408   21499
88878   49200
12559   91603
51392   13481
35286   97345
84379   67963
93438   91603
60752   53106
19987   81076
63280   35027
43747   71810
22803   66237
75256   75256
70441   72003
77992   57743
70884   21412
91936   77547
39673   53183
31518   19763
23066   48344
45455   77547
58606   32254
33075   21971
12200   19763
42764   79555
45004   94858
87970   98880
52319   38436
81837   86922
67858   57654
86460   95056
";

    // Parse the input string into a vector of tuples
    let input: Vec<(i32, i32)> = example
        .lines()
        .map(|line| {
            // Split the line by whitespace, collect into a vector
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Failed to parse number"))
                .collect();
            
            // Ensure we have exactly two numbers
            if nums.len() != 2 {
                panic!("Invalid input line: {}", line);
            }
            
            (nums[0], nums[1])
        })
        .collect();

    // Count occurrences in the second column
    let mut second_column_counts: HashMap<i32, usize> = HashMap::new();
    for (_, b) in &input {
        *second_column_counts.entry(*b).or_insert(0) += 1;
    }

    // Print second column counts for debugging
    println!("Occurrences in Second Column:");
    for (num, count) in &second_column_counts {
        println!("{}: {} times", num, count);
    }

    // Calculate result by multiplying first column key with its count in second column
    let final_result: Vec<(i32, i32, usize)> = input.iter()
        .map(|&(a, b)| {
            let count = *second_column_counts.get(&a).unwrap_or(&0);
            println!("Debug: a = {}, b = {}, count = {}, result = {}", a, b, count, a * count as i32);
            (a, a * count as i32, count)
        })
        .collect();

    // Print final results
    println!("\nFirst Column Key * Count of Second Column:");
    for (key, result, count) in &final_result {
        println!("{} * {} = {}", key, count, result);
    }

    // Sum of final results
    let sum_of_results: i32 = final_result.iter().map(|&(_, b, _)| b).sum();
    println!("\nSum of Results: {}", sum_of_results);

    Ok(())
}