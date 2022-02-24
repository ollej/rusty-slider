# Application Events

Olle Wreede

---

## Application Events

* Publish/subscribe pattern
* Decouple logic with domain specific events
* Triggered on actions in the system
* Subscribed handlers perform asynchronously

---

## Use Cases

* Statistics
* Billing
* Audit log
* Extract side-effects (emails, notifications etc)
* Decoupling between modules
* Separate external concerns

---

## Events

* Described as a verb in past tense
* Contains domain specfic data
* Includes no logic
* Plain Ruby object

---

## Handlers

* Subscribes to events
* Runs asynchronously using Sidekiq
* Has to be idempotent
* Contains logic

---

## ApplicationEvent

* Events classes extends from Events::ApplicationEvent
* Includes a payload hash
* May have one or more categories
* Categories may require attributes
* Always includes attributes uuid and created_at

---

## EventHandler

* Sub-classes Events::EventHandler
* Subscribes to events and/or categories
* May define priority queue (critical/default/low)
* handle_event method is called with event instance

---

## Example Event

```ruby
class UserLoggedIn < Events::ApplicationEvent
  include Events::UserCategory

  def initialize(attributes = {})
    super
    @payload[:login_at] = DateTime.now.utc.to_s
  end

  def visit_at
    @payload[:login_at]
  end
end
```

---

## Example Category

```ruby
module Events
  module UserCategory
    def user=(user)
      @payload[:user] = user.uuid
    end

    def user
      User.find_by!(uuid: @payload[:user])
    end
  end
end
```

---

## Example Handler

```ruby
class TrackLogins < Events::EventHandler
  queue Events::QUEUE_LOW
  attach_to UserLoggedIn

  def handle_event(event)
    File.open('/tmp/logins', 'a') do |file|
      file.write("#{user.name} logged in on #{event.login_at}\n")
    end
  end
end
```

---

# Olle Wreede

@ollej
