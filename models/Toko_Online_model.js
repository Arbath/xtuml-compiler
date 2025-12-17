// Model Name: Toko Online
// version: 1.0

export class Customer {
    // Domain Reference: UserOrder
    
    constructor(id, name, email, registered_at) {
        this.id = id;
        this.name = name;
        this.email = email;
        this.registered_at = registered_at;
    }
    
    get_order_history() {
        // Implies relationship: R1 'placed by' Customer(1) -> Order(M)
        return Orders.filter(o => o.customer_id === this.id);
    }
    
    update_email() {
        this.email = new_email;
        // TODO: raise event CustomerEmailUpdated
    }
    
    // Event: CustomerEmailUpdated -> send_verification_email()
}


/* =========================
   STATE ENUM FOR Order
   ========================= */
const OrderState = Object.freeze({
    Created: "Created",
    Confirmed: "Confirmed",
    Shipped: "Shipped"
});

export class Order {
    constructor(id, customer_id, status, created_at, total_amount) {
        this.id = id;
        this.customer_id = customer_id;
        this.status = status;
        this.created_at = created_at;
        this.total_amount = total_amount;
        this._state = OrderState.Created;
    }
    
    confirm_order() {
        this.status = 'confirmed';
        // TODO: raise event OrderConfirmed
    }
    
    calculate_total() {
        // Implies relationship: R2 'contains' Order(1) -> OrderItem(M)
        let total = 0;
        const items = OrderItems.filter(item => item.order_id === this.id);
        
        for (const item of items) {
          total += item.get_subtotal();
        }
        
        this.total_amount = total;
    }
    
    ship_order() {
        if (this.status === 'confirmed') {
          this.status = 'shipped';
          // TODO: raise event OrderShipped
        }
    }
    

    /* =========================
       STATE MACHINE
       ========================= */


    OrderConfirmed() {
        switch (this._state) {
            case OrderState.Created:
                this._enterConfirmed();
                break;
            default:
                this._invalidTransition("OrderConfirmed");
        }
    }

    OrderShipped() {
        switch (this._state) {
            case OrderState.Confirmed:
                this._enterShipped();
                break;
            default:
                this._invalidTransition("OrderShipped");
        }
    }

    _enterCreated() {
        this._state = OrderState.Created;
    }

    _enterConfirmed() {
        this.calculate_total();
        this._state = OrderState.Confirmed;
    }

    _enterShipped() {
        this.notify_customer_shipping();
        this._state = OrderState.Shipped;
    }

    get state() {
        return this._state;
    }

    _invalidTransition(event) {
        throw new Error(
            `Invalid event ${event} in state ${this._state}`
        );
    }

    // Event: OrderConfirmed -> status = 'confirmed'
    // Event: OrderShipped -> notify_customer_shipping()
}

export class OrderItem {
    constructor(id, order_id, product_id, quantity, price_per_unit) {
        this.id = id;
        this.order_id = order_id;
        this.product_id = product_id;
        this.quantity = quantity;
        this.price_per_unit = price_per_unit;
    }
    
    get_subtotal() {
        return this.quantity * this.price_per_unit;
    }
    
    get_product_name() {
        // Implies relationship: R3 'references' OrderItem(M) -> Product(1)
        const product = Products.get({ id: this.product_id });
        return product.name;
    }
    
}

export class Product {
    constructor(id, sku, name, description, price) {
        this.id = id;
        this.sku = sku;
        this.name = name;
        this.description = description;
        this.price = price;
    }
    
    get_inventory_level() {
        // Implies relationship: R4 'has' Product(1) -> Inventory(1)
        const inventory = Inventory.get({ product_id: this.id });
        return inventory.stock_level;
    }
    
    update_price() {
        this.price = new_price;
        // TODO: raise event ProductPriceChanged
    }
    
    // Event: ProductPriceChanged -> notify_wishlist_users()
}

export class Inventory {
    constructor(id, product_id, stock_level, location) {
        this.id = id;
        this.product_id = product_id;
        this.stock_level = stock_level;
        this.location = location;
    }
    
    decrement_stock() {
        if (this.stock_level >= amount) {
          this.stock_level -= amount;
        
          if (this.stock_level < 10) {
            // TODO: raise event LowStockWarning
          }
        } else {
          // TODO: raise event OutOfStockError
        }
    }
    
    increment_stock() {
        this.stock_level += amount;
    }
    
    send_error_notification() {
        // select user where user equal to user
    }
    
    // Event: LowStockWarning -> create_purchase_order()
    // Event: OutOfStockError -> notify_sales_team(), send_error_notification()
}

// Event Dispatcher
export class EventBus {
    static dispatch(eventName, payload) {
        console.log(`[EVENT] ${eventName}`, payload);
    }
}

// Event Definitions
export const Events = {

  "OrderConfirmed": {
      trigger: "Order",
      action: "status = 'confirmed'"
  },

  "CustomerEmailUpdated": {
      trigger: "Customer",
      action: "send_verification_email()"
  },

  "OrderShipped": {
      trigger: "Order",
      action: "notify_customer_shipping()"
  },

  "ProductPriceChanged": {
      trigger: "Product",
      action: "notify_wishlist_users()"
  },

  "LowStockWarning": {
      trigger: "Inventory",
      action: "create_purchase_order()"
  },

  "OutOfStockError": {
      trigger: "Inventory",
      action: "notify_sales_team(), send_error_notification()"
  }

};


// Generated by: xtuml-compiler