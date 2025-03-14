
. test-vectors/dkms.sh

$dkms identifier init -a exp --witness-url http://w1.ea.argo.colossi.network --watcher-url http://wa1.ea.argo.colossi.network

OOBI='[{"eid":"BNJJhjUnhlw-lsbYdehzLsX1hJMG9QJlK_wJ5AunJLrM","scheme":"http","url":"http://w1.ea.argo.colossi.network/"},{"cid":"EJc1zF0AZJaHsctmnaSwhcZt48Fh6I-P8l62fFxTvexI","role":"witness","eid":"BNJJhjUnhlw-lsbYdehzLsX1hJMG9QJlK_wJ5AunJLrM"}]'
SIGNED_1_SIG='{"hello":"there"}-FABEJc1zF0AZJaHsctmnaSwhcZt48Fh6I-P8l62fFxTvexI0AAAAAAAAAAAAAAAAAAAAAAAEJc1zF0AZJaHsctmnaSwhcZt48Fh6I-P8l62fFxTvexI-AABAACGEwJrd5_l2wkkBij3ZsWvOZ4OUXk1PZt-gDOKqtytucQJKGuWvrOq9eRrMh_9xXibC1CHGKainoj6auSOw3wH'

SIGNED_2_SIG='{"hello":"there"}-FABEJc1zF0AZJaHsctmnaSwhcZt48Fh6I-P8l62fFxTvexI0AAAAAAAAAAAAAAAAAAAAAAAEJc1zF0AZJaHsctmnaSwhcZt48Fh6I-P8l62fFxTvexI-AACAACGEwJrd5_l2wkkBij3ZsWvOZ4OUXk1PZt-gDOKqtytucQJKGuWvrOq9eRrMh_9xXibC1CHGKainoj6auSOw3wHABBPxn33XYD_MR1r-xkBHR-Lw-ffupPrfh8lbqBGKtUQFVEZ7SG0PQVXoswn1ZiKtflVWJPB0ZD1JeqJvZHd61kC'

SIGNED_2_SIG_WRONG='{"hello":"there"}-FABEJc1zF0AZJaHsctmnaSwhcZt48Fh6I-P8l62fFxTvexI0AAAAAAAAAAAAAAAAAAAAAAAEJc1zF0AZJaHsctmnaSwhcZt48Fh6I-P8l62fFxTvexI-AACAACGEwJrd5_l2wkkBij3ZsWvOZ4OUXk1PZt-gDOKqtytucQJKGuWvrOq9eRrMh_9xXibC1CHGKainoj6auSOw3wHABBPxn33XYD_MR1r-xkBHR-Lw-ffupPrfh8lbqBGKtUQFVEZ7SG0PQVXoswn1ZiKtflVWJPB0ZD1JeqJvZHd61kR'

$dkms data verify -a exp -m "$SIGNED_1_SIG" -o "$OOBI"

$dkms data verify -a exp -m "$SIGNED_2_SIG_WRONG" -o "$OOBI"

$dkms data verify -a exp -m "$SIGNED_2_SIG" -o "$OOBI"
