# Group-35

Backup Rust APPUNTI INIZIALI

Attività da fare:
Riconoscimento forma con il mouse
App deve attivarsi in background
Interfaccia grafica
- almeno per la conferma di attivazione (Box con un timer)
- Finestra per la configurazione iniziale (in alternativa alla CLI)


Workflow:
Faccio il primo segno con il mouse
Appare finestra a schermo con countdown
- Se faccio il secondo segno entro la scadenza, parte il segnale acustico con il backup
- Se non faccio niente o non indovino il segno, la finestra scompare alla scadenza
  Al termine del backup c’è un nuovo segnale acustico e viene salvato il log nella USB

Le info che servono per la configurazione (da inserire in CLI o in GUI se si riesce):
- Avvio del tool in fase di boostrap SI/NO
- Cartella sorgente del backup e tipo di file
- Drive di destinazione ( con controllo ad avvio del sistema, controllo alla scrittura del log ogni 2 minuti, eventuale bip di errore all’attivazione del backup)
- Scelto del Segno da fare con il mouse (opzionale)
- Directory dove salvare il log di sistema

## COSA RIMANE DA FARE:
- Check: se il drive esterno non è più presente, annulla il backup sennò va in panic OK
- Gestire la X o il tasto chiudi per inserire i valori di default nella CSV
- Rendere dinamici i parametri delle funzioni per backup (forme e path) OK, CHECK LOG
- Pop Up di conferma: dopo la prima conferma c’è un pop up che scompare dopo 10sec
- Sistemare Usize / i32 nella funzione del log
- Test eseguibili per le diverse architetture OK
- Rimuovere "Boostrap" dalla GUI e inserire una lista di filtri di file (es. all, pdf, mp4..)


