/** --feature-flag feature:feature-2, type: ui-component, priority: high */

export function TestComponent() {
  return {
    render: function() {
      console.log("Testing feature 2");
    }
  };
}

// --feature-flag feature:feature-2, type: utility, status: active

function helperFunction() {
  return "helper";
}
