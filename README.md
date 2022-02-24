# Un problème de psychométrie


## **Énoncé**

L’équipe Science souhaite faire passer le Questionnaire Big Five GetPro du format Likert au format ipsatif. Pour cela, l’équipe Science doit apparier les items (questions) du questionnaire, dans un premier temps, sous forme de quadruplets. Connaissant l’équipe Science, Terry sait que d’ici quelques mois il faudra résoudre le problème d’appariement à nouveau mais pour un ensemble de triplets et par la suite pour des paires. Terry a donc informé l’équipe Science que la résolution de ce problème était possible par l’algorithmie. 

Vous venez d’être recruté chez GetPro et votre première tâche est donc d’aider l’équipe Science à résoudre leur problème tout en créant un algorithme qui permettra de résoudre les problèmes similaires à venir.


## **Données**

Le questionnaire est constitué de :



* 5 domaines (**A**gréabilité, **C**onscienciosité,** E**xtraversion, **N**évrotisme, **O**uverture)
* 30 facettes, soit 6 par domaine (A1, A2, A3, A4, A5, A6, C1 … O4, O5, O6)
* 300 questions/items, soit 10 par facette (A1.0, A1.1, …O6.7, O6.8, O6.9)

L’objectif principal de l’appariement est de comparer chaque facette à toutes les autres en réunissant les items au sein d’un total de 75 quadruplets (75 = 300/4).

(i) En ce qui concerne les **domaines**, l’équipe Science souhaite éviter les quadruplets contenant des items tous issus d’un même domaine. Un quadruplet au format (A,A,A,A) est donc interdit et on favorise les quadruplets ne contenant qu’un ou deux items issus du même domaine (un maximum de format de type (A,C,E,N) ; moins de quadruplet de type (A,A,C,E) ; et un minimum de quadruplets de type (A,A,A,C)), la qualité de l’algo sera déterminée en fonction de ces critères.

(ii) En ce qui concerne les **facettes**, l’idéal est de comparer chaque facette au moins une fois aux 29 autres (A1 doit être, dans l’idéal, comparée à A2, A3 … O4, O5 **et** O6). Si cela s’avère impossible, il est possible de ne pas comparer une facette à certaines des autres facettes mais dans ce cas, une facette doit impérativement (au moins) être comparée aux autres facettes de son domaine (A1 doit _a minima_ être comparée à A2, A3, A4, A5 et A6). **Tout l’objectif de l’appariement est de réaliser un maximum de comparaisons entre facettes.** Par ailleurs, il est interdit de comparer une facette à elle-même (A1 ne peut être comparée à A1), il ne faut donc pas d’items issus d’une même facette au sein d’un même quadruplet. En revanche, il est possible de comparer plusieurs fois deux mêmes facettes.

Sur la page suivante vous trouverez des questions pour guider le début du raisonnement, vous pouvez ou non y répondre pour vous aider dans vos réflexions.

Bon courage !



1. **Raisonnement général**
1. Combien de comparaisons sont réalisées au sein d’un quadruplet ? Combien de comparaisons sont donc réalisées au sein de 75 quadruplets ?
2. Combien de comparaisons sont nécessaires au total pour que chaque facette soit comparée une fois à chacune des autres ? Ce résultat est-il plus petit que le nombre de comparaisons réalisées au sein de 75 quadruplets ? Est-ce un problème ou un avantage ?
3. De combien est la différence entre le nombre de comparaisons réalisées et le nombre de comparaisons nécessaires ? Pensez-vous qu’il soit possible de remplir les objectifs idéaux de l’équipe Science ? Si non, quel objectif pensez-vous revoir à la baisse en priorité ?
2. **(Optionnel) Cas des triplets et des paires**
1. Adaptez votre algorithme au cas des triplets : 100 triplets à réaliser. 
2. Adaptez votre algorithme au cas des doublets : 150 doublets à réaliser. 
3. Quelle est la conséquence en termes de comparaisons ? Pensez-vous que l’équipe Science ait bien fait de choisir des quadruplets ? 