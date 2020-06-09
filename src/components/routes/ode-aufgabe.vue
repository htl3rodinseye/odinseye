<template>
  <ode-box id="ode-Aufgabe" class="">
    <h1>{{task.name}}</h1>
    <span>Status: {{task.status}}</span>
  </ode-box>
</template>

<script>

  import OdeBox from "../ode-box";
  export default {
    name: "ode-aufgabe",
    components: {OdeBox},
    data() {
      return {}
    },
    created() {
      this.getTask()
    },
    methods: {
      getTasks: async function () {
        console.log('Aufgabe/getTasks()');
        console.log(this.$router.id);
        let url = 'http://caretaker.wurzer.cc:9040/exercises/';
        try {
          console.log('Aufgabe/getTasks(): Data loaded');

          let resp = await fetch(url + '?exid=' + this.$router.id).then(response => {
            return response.json()
          });
          this.task = resp;
          /*this.tasks = JSON.parse('{"0":{"id":"1","name":"Uebung 1 - Wie bekommt \\"man\\" Hilfe?","description":"Schauen Sie sich mit dem Kommando man die Manual Pages zu folgenden Befehlen an: ls, pwd, cat, cp, mv, rm, mkdir, rmdir. Beispiel: >man ls"},"1":{"id":"2","name":"Uebung 2 - Ordner auflisten","description":"Listen Sie mit dem Kommando ls ihr aktuelles Verzeichnis auf."},"2":{"id":"3","name":"Uebung 3 - Command line options","description":"Verschiedene Befehle haben command line options welche mit einem - (Bindestrich) an dem Kommando hinzugefuegt werden koennen.Geben sie ls -l ein. -l ist die Funktion, mit welcher die Ausgabe im Long Listing Format ausgegeben wird."},"3":{"id":"4","name":"Uebung 4 - Absoluter Pfadname","description":"Listen Sie mit dem Kommando pwd den absoluten Pfadnamen des aktuellen Verzeichnisses auf. Ihr Nutzerverzeichnis finden Sie unter dem home Verzeichnis."},"4":{"id":"5","name":"Uebung 5 - Standard line output","description":"Geben sie ihre Zeitzone mit dem Befehl cat \\/etc\\/timezone aus dem timezone Datei aus."},"5":{"id":"6","name":"Uebung 6 - Verzeichnisse erstellen 1","description":null},"6":{"id":"7","name":"Uebung 7 - Verzeichnisse erstellen 2","description":"Listen Sie mit dem Kommando pwd den absoluten Pfadnamen des aktuellen Verzeichnisses auf.Ihr Nutzerverzeichnis finden Sie unter dem home Verzeichnis. "},"7":{"id":"8","name":"Uebung 8 - Kopieren","description":"Der Befehl cp wird verwendet, um Dateien oder Order zu kopieren. Kopieren sie dir21 von der letzten Uebung in dir1."},"8":{"id":"9","name":"Uebung 9 - Verschieben","description":"Der Befehl mv wird verwendet, um Dateien oder Order zu verschieben. Verschieben sie dir21 von der letzten Uebung in das Verzeichnis \\/home\\/odin."},"9":{"id":"10","name":"Uebung 10 - Loeschen von Verzeichnissen und Dateien","description":"Der Befehl rm wird verwendet, um Dateien oder Order zu loeschen. Doch ist dieser Befehl sehr stark und fuer diese Uebung nicht verwendet.Loesche mit dem Befehl rmdir das Verzeichnis dir12 im \\/home\\/odin Verzeichnis."}}')*/
          console.log(this.task);

          console.log('Aufgaben/getTasks(): Data processed');

          this.getGroups()
        } catch (err) {
          console.error(err)
        }
      }
    }
  }
</script>

<style scoped>

</style>
