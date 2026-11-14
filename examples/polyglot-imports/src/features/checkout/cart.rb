require "json"
require_relative "../payments/gateway"

class Cart
  def checkout(total)
    Gateway.new.charge(total)
  end
end
