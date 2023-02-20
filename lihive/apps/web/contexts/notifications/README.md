# Notification Provider

This is a React context provider that allows you to interact with lihive
notifications from anywhere within the app.

Wrap the app in the context provider (in `_app.tsx`):

```tsx
function App() {
  return <NotificationProvider>
    {...}
  </NotificationProvider>
}
```

Use the `useNotifications()` hook to retrieve the current notifications and
related functionality:

```tsx
const { 
  notifications, // an array of Notification object
  unread,        // the number of notifications NOT marked as read
  markRead       // a callback you can use to mark notifications as read
} = useNotifications();

// the markRead callback accepts an array of notification ids to mark as read so
// here we create a callback to mark all notifications as read
const markAllRead = () => {
  markRead(notifications.map(n => n.id));
}

// render the notifications
return <List>
  {notifications.map(n => (<Item>{...}</Item>))}
</List>

// or let the user mark them as read
return <Button onClick={markAllRead}/>

// or whatever else you'd like to do!
```