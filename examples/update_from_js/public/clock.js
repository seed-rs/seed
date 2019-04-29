/**
 * Fire event that can be handled by Seed application
 *
 * @param {string} msgName Msg variant name, e.g. Tick(String) will be "Tick"
 * @param {*|undefined} data Serialized Msg variant data
 */
const triggerUpdate = (msgName, msgData) => {
  const event = new CustomEvent("triggerupdate", {
    detail: msgData === undefined ? msgName : { [msgName]: msgData }
  });
  window.dispatchEvent(event);
};

const enableClock = () => {
  const triggerTickEvent = () => {
    triggerUpdate("Tick", new Date().toLocaleTimeString());
  };
  triggerTickEvent();

  setInterval(() => {
    triggerTickEvent();
  }, 1000);

  triggerUpdate("ClockEnabled");
};
