<template>
  <div id="ode-info" class="grid">
    <ode-box id="ode-info-text"
             title="Was ist Odins Eye">
      <p>
        Odins Eye ist eine Lernplattform, um die Grundzüge der Linux Bash zu erlernen. Dieses Projekt entsteht momentan
        im Auftrag der HTL Rennweg im Rahmen des ITP Unterrichts.
      </p>
    </ode-box>
    <ode-box id="ode-info-download" title="Download">
      <p>
        Die heruntergeladene Binary Datei kann jetzt per Bash ausgeführt werden.<br>
        Dazu muss nur der Name der Binary Datei eingegeben werden.<br>
        <span class="monospaced">User> odins_eye</span>
      </p>
      <div class="row wrapper" style="justify-content: space-between">
        <i class="fab fa-linux"/>
        <a class="button" href="./static/odins_eye" download><i class="fas fa-download"/> Download</a>
      </div>
      <a href="https://github.com/htl3rodinseye/odinseye"><i class="fab fa-github"/></a>
    </ode-box>
    <ode-box id="ode-info-stand" title="Aktueller Stand">
      <div>
        <ode-ladeleiste :percentage="70" color="#DD382F" title="Website"/>
        <ode-ladeleiste :percentage="70" color="#744390" title="Backend"/>
        <ode-ladeleiste :percentage="75" color="#73A6FF" title="Datensätze"/>
        <ode-ladeleiste :percentage="80" color="#60CD54" title="Linux Bash Anwendung"/>
      </div>
    </ode-box>
  </div>
</template>

<script>
  import OdeBox from "../ode-box";
  import OdeLadeleiste from "../ode-ladeleiste";

  export default {
    name: "ode-info",
    components: {OdeLadeleiste, OdeBox}
  }
</script>

<style scoped>

  #ode-info {
    grid-template-columns: 1fr 1fr 1fr 1fr 1fr;
    grid-template-rows: auto;
  }

  #ode-info-text {
    grid-column: span 4;
  }

  #ode-info-download {

  }

  #ode-info-download div {
    font-size: 5em;
  }

  /*#ode-info-download a {
    font-size: 3.5em;
    width: 100%;
    margin: auto;
  }*/

  #ode-info-stand {
    grid-column: span 5;
  }


</style>
