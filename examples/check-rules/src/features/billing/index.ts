// --feature-flag name: new-invoice-layout, introduced_on: 2020-01-01
import { addToCart } from '../checkout/cart';
import { Button } from '../shared/ui';
// --feature-flag name: future-flag, introduced_on: 2999-01-01

export function billing() {
  return addToCart(Button);
}
